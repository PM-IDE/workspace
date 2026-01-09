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
  private const char Separator = ' ';

  private record Event(string Message, string Group) : IPointData
  {
    public Point Point => default;
  }

  private class EventsIndex(List<Event> events, SortedList<string, int> index) : ISpatialIndex<PointInfo<Event>>
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

      var firstWord = ConvertMessageToWord(point.Item.Message, index);

      foreach (var evt in myEventsByGroups[point.Item.Group])
      {
        if (ReferenceEquals(evt, point))
        {
          continue;
        }

        var secondWord = ConvertMessageToWord(evt.Item.Message, index);
        if (secondWord.Length != firstWord.Length) continue;

        var distance = CalculateEditDistance(firstWord, secondWord);

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


  [GeneratedRegex("rustc_[a-z]+::[a-z]+")]
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

    AnsiConsole.MarkupLine(
      $"[green]Processed event:[/] [gray]{Markup.Escape(line)}[/], group [bold]{Markup.Escape(group.ToString())}[/]");
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

    var index = new SortedList<string, int>(
      myEvents.SelectMany(e => e.Message.Split(Separator))
        .ToHashSet()
        .Select((e, index) => (e, index)).ToDictionary(p => p.e, p => p.index)
    );

    var clusters = Dbscan.Dbscan.CalculateClusters(new EventsIndex(myEvents, index), 3, 1);
    foreach (var cluster in clusters.Clusters)
    {
      if (cluster.Objects.Count is 0) continue;

      AnsiConsole.MarkupLine("[blue]CLUSTER[/]");

      var lcs = ConvertMessageToWord(cluster.Objects[0].Message, index);
      foreach (var obj in cluster.Objects.Skip(1))
      {
        AnsiConsole.WriteLine(obj.Message);

        lcs = FindLcs(ConvertMessageToWord(obj.Message, index), lcs);
      }

      AnsiConsole.Markup("[blue]LCS:[/] ");
      foreach (var idx in lcs)
      {
        Console.Write($"{index.GetKeyAtIndex(index.IndexOfValue(idx))} ");
      }

      AnsiConsole.WriteLine();
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

  public static T[] FindLcs<T>(ReadOnlySpan<T> first, ReadOnlySpan<T> second)
    where T : IEqualityOperators<T, T, bool>
  {
    var n = first.Length;
    var m = second.Length;
    var dp = new int[n + 1, m + 1];

    for (var i = 1; i <= n; i++)
    {
      for (var j = 1; j <= m; j++)
      {
        if (first[i - 1] == second[j - 1])
        {
          dp[i, j] = dp[i - 1, j - 1] + 1;
        }
        else
        {
          dp[i, j] = Math.Max(dp[i - 1, j], dp[i, j - 1]);
        }
      }
    }

    return RestoreLcs(first, second, dp, n, m);
  }

  private static int[] ConvertMessageToWord(string message, SortedList<string, int> index) =>
    message.Split(Separator).Select(word => index[word]).ToArray();

  public static T[] RestoreLcs<T>(ReadOnlySpan<T> x, ReadOnlySpan<T> y, int[,] dp, int n, int m)
    where T : IEqualityOperators<T, T, bool>
  {
    int i = n, j = m;
    List<T> lcs = [];

    while (i > 0 && j > 0)
    {
      if (x[i - 1] == y[j - 1])
      {
        lcs.Add(x[i - 1]);
        i--;
        j--;
      }
      else if (dp[i - 1, j] > dp[i, j - 1])
      {
        i--;
      }
      else
      {
        j--;
      }
    }

    lcs.Reverse();

    return lcs.ToArray();
  }
}