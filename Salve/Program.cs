using System.ComponentModel;
using System.Diagnostics;
using System.Numerics;
using System.Text.RegularExpressions;
using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Utils;
using Bxes.Writer.Stream;
using Dbscan;
using JetBrains.Annotations;
using Spectre.Console;
using Spectre.Console.Cli;

var app = new CommandApp();
app.Configure(cfg => { cfg.AddCommand<SerializeOutputToBxesCommand>("serialize-to-bxes"); });

app.Run(args);


internal enum ParserKind
{
  Rustc
}

[UsedImplicitly]
internal class SerializeOutputToBxesCommand : Command<SerializeOutputToBxesCommand.Settings>
{
  [UsedImplicitly]
  public class Settings : CommandSettings
  {
    [CommandArgument(0, "<parser>")]
    [Description("The parser which should be used to parse command output")]
    public required ParserKind ParserKind { get; init; }

    [CommandArgument(1, "<o>")]
    [Description("The output path of a bXES file")]
    public required string OutputFilePath { get; init; }

    [CommandArgument(2, "<exec>")]
    [Description("Command executable")]
    public required string Executable { get; init; }

    [CommandOption("--args")]
    [Description("Command arguments")]
    public required string? Arguments { get; init; }

    [CommandOption("--workdir")]
    [Description("Working directory")]
    public required string? WorkingDirectory { get; init; }
  }


  protected override int Execute(CommandContext context, Settings settings, CancellationToken cancellationToken)
  {
    try
    {
      var directory = Path.GetDirectoryName(settings.OutputFilePath);
      if (!Directory.Exists(directory))
      {
        throw new Exception($"Directory {directory} does not exist");
      }

      PathUtil.EnsureDeleted(settings.OutputFilePath);

      var info = new ProcessStartInfo
      {
        FileName = settings.Executable,
        RedirectStandardOutput = true,
        RedirectStandardError = true,
        WorkingDirectory = settings.WorkingDirectory,
        Arguments = settings.Arguments,
        CreateNoWindow = true
      };

      var process = new Process
      {
        StartInfo = info
      };

      var processor = LogsProcessorFactory.Create(settings.ParserKind, settings.OutputFilePath);
      processor.Initialize();

      try
      {
        // ReSharper disable once AccessToDisposedClosure
        process.OutputDataReceived += (_, args) => processor.Process(args.Data);
        // ReSharper disable once AccessToDisposedClosure
        process.ErrorDataReceived += (_, args) => processor.Process(args.Data);

        if (!process.Start())
        {
          throw new Exception("Failed to start process");
        }

        process.BeginOutputReadLine();
        process.BeginErrorReadLine();

        process.WaitForExit();
      }
      finally
      {
        processor.Dispose();
      }

      return 0;
    }
    catch (Exception ex)
    {
      AnsiConsole.WriteException(ex);
      return 1;
    }
  }
}

internal static class LogsProcessorFactory
{
  public static ILogsProcessor Create(ParserKind parserKind, string outputPath) => parserKind switch
  {
    ParserKind.Rustc => new RustcLogsParser(outputPath),
    _ => throw new ArgumentOutOfRangeException(nameof(parserKind), parserKind, null)
  };
}

internal interface ILogsProcessor : IDisposable
{
  void Initialize();
  void Process(string? line);
}

internal partial class RustcLogsParser(string outputPath) : ILogsProcessor
{
  private record Event(string Message, string Group) : IPointData
  {
    public Point Point => default;
  }

  private class EventsIndex(List<Event> events) : ISpatialIndex<PointInfo<Event>>
  {
    private readonly Dictionary<string, List<PointInfo<Event>>> myEventsByGroups =
      events
        .GroupBy(e => e.Group)
        .ToDictionary(e => e.Key, e => e.Select(evt => new PointInfo<Event>(evt)).ToList());


    public IReadOnlyList<PointInfo<Event>> Search() => myEventsByGroups.Values.SelectMany(v => v).ToList();

    public IReadOnlyList<PointInfo<Event>> Search(in IPointData p, double epsilon)
    {
      var point = (PointInfo<Event>)p;
      var result = new List<PointInfo<Event>>();

      foreach (var evt in myEventsByGroups[point.Item.Group])
      {
        if (ReferenceEquals(evt, point))
        {
          continue;
        }

        var distance = CalculateEditDistance(point.Item.Message.AsSpan(), evt.Item.Message.AsSpan());
        if (distance <= epsilon)
        {
          result.Add(evt);
        }
      }

      return result;
    }

    private static int CalculateEditDistance<T>(ReadOnlySpan<T> first, ReadOnlySpan<T> second) where T : IEqualityOperators<T, T, bool>
    {
      if (first.Length == 0) return second.Length;
      if (second.Length == 0) return first.Length;

      var current = 1;
      var previous = 0;

      var r = new int[2, second.Length + 1];
      for (var i = 0; i <= second.Length; i++)
      {
        r[previous, i] = i;
      }

      for (var i = 0; i < first.Length; i++)
      {
        r[current, 0] = i + 1;
        for (var j = 1; j <= second.Length; j++)
        {
          var cost = (second[j - 1] == first[i]) ? 0 : 1;
          r[current, j] = Min(r[previous, j] + 1, r[current, j - 1] + 1, r[previous, j - 1] + cost);
        }

        previous = (previous + 1) % 2;
        current = (current + 1) % 2;
      }

      return r[previous, second.Length];
    }

    private static int Min(int e1, int e2, int e3) => Math.Min(Math.Min(e1, e2), e3);
  }


  [GeneratedRegex("rustc_([a-z])+::([a-z])+")]
  private static partial Regex MessageGroupRegex();

  private readonly SingleFileBxesStreamWriterImpl<InMemoryEventImpl> myWriter = new(outputPath, 1);
  private readonly Lock myLock = new();
  private readonly List<Event> myEvents = [];

  private volatile bool myIsDisposed;


  public void Initialize() => myWriter.HandleEvent(new BxesTraceVariantStartEvent(1, []));

  public void Process(string? line)
  {
    if (myIsDisposed || line is null) return;

    line = line.Trim();

    if (!ShouldProcess(line, out var group))
    {
      AnsiConsole.Markup("[yellow]Skipping line:[/]");
      AnsiConsole.WriteLine(line);
      return;
    }

    using (myLock.EnterScope())
    {
      if (myIsDisposed)
      {
        AnsiConsole.MarkupLine($"[red]The writer is disposed, will not write event [/] {line}");
        return;
      }

      myEvents.Add(new Event(line, group.ToString()));
    }

    AnsiConsole.MarkupLine($"[green]Processed event:[/] [gray]{line}[/], group [bold]{group}[/]");
  }

  private static bool ShouldProcess(string line, out ReadOnlySpan<char> messageGroup)
  {
    messageGroup = default;

    if (!line.StartsWith("INFO")) return false;
    if (MessageGroupRegex().Match(line) is not { } match) return false;

    messageGroup = match.ValueSpan;

    return true;
  }

  public void Dispose()
  {
    using var _ = myLock.EnterScope();

    var clusters = Dbscan.Dbscan.CalculateClusters(new EventsIndex(myEvents), 3, 1);
    foreach (var cluster in clusters.Clusters)
    {
      AnsiConsole.MarkupLine("[blue]CLUSTER[/]");
      foreach (var obj in cluster.Objects)
      {
        AnsiConsole.WriteLine(obj.Message);
      }

      AnsiConsole.WriteLine();
    }

    foreach (var @event in myEvents)
    {
      var bxesEvent = new InMemoryEventImpl(DateTime.UtcNow.Ticks, new BxesStringValue(@event.Message), []);
      myWriter.HandleEvent(new BxesEventEvent<InMemoryEventImpl>(bxesEvent));
    }

    try
    {
      myWriter.Dispose();
      AnsiConsole.WriteLine("Disposed writer");
    }
    finally
    {
      myIsDisposed = true;
    }
  }
}