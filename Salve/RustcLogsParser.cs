using System.Text;
using System.Text.RegularExpressions;
using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Writer.Stream;
using Dbscan;
using Spectre.Console;
using WordsIndex = System.Collections.Generic.SortedList<string, int>;

namespace Salve;

internal partial class RustcLogsParser(
  string outputPath,
  bool useGroupsAsEventNames,
  int maxTokensInEvent,
  bool leaveOnlyMethodEvents)
  : ILogsProcessor
{
  private const char Separator = ' ';

  [GeneratedRegex("rustc(_[a-z]+)+(::[a-z_]+)*")]
  private static partial Regex FqnRegex();

  [GeneratedRegex("[0-9]+ms")]
  private static partial Regex MsRegex();


  private readonly SingleFileBxesStreamWriterImpl<InMemoryEventImpl> myWriter = new(outputPath, 1);
  private readonly Lock myLock = new();
  private readonly List<Event> myEvents = [];

  private volatile bool myIsDisposed;


  public void Initialize() => myWriter.HandleEvent(new BxesTraceVariantStartEvent(1, []));

  public void Process(string? line)
  {
    if (myIsDisposed || line is null) return;

    line = line.Trim();

    var kind = (FqnRegex().Match(line) is { Index: 0, Length: > 0 }) switch
    {
      true => EventKind.Method,
      false => EventKind.Message
    };

    if (leaveOnlyMethodEvents && kind is EventKind.Message)
    {
      LogSkippedLine(line);
      return;
    }

    line = MsRegex().Replace(line, string.Empty).Trim();

    if (!ShouldProcess(line, kind, out var group))
    {
      LogSkippedLine(line);
      return;
    }

    using (myLock.EnterScope())
    {
      if (myIsDisposed)
      {
        AnsiConsole.MarkupLine($"[red]The writer is disposed, will not write event [/] {line}");
        return;
      }

      var groupStr = group.ToString();
      var name = kind switch
      {
        EventKind.Method => groupStr,
        EventKind.Message => line,
        _ => throw new ArgumentOutOfRangeException()
      };

      myEvents.Add(new Event(kind, name, groupStr));
    }

    AnsiConsole.MarkupLine(
      $"[green]Processed event:[/] [gray]{Markup.Escape(line)}[/], group [bold]{Markup.Escape(group.ToString())}[/]");
  }

  private static void LogSkippedLine(string line)
  {
    AnsiConsole.Markup("[yellow]Skipping line:[/]");
    AnsiConsole.WriteLine(line);
  }

  private static bool ShouldProcess(string line, EventKind kind, out ReadOnlySpan<char> eventGroup)
  {
    eventGroup = default;

    if (kind is EventKind.Method)
    {
      eventGroup = FqnRegex().Match(line).ValueSpan;
      return true;
    }

    if (!line.StartsWith("INFO") && !line.StartsWith("DEBUG")) return false;
    if (FqnRegex().Match(line) is not { } match) return false;

    eventGroup = match.ValueSpan;

    return true;
  }

  public void Dispose()
  {
    using var _ = myLock.EnterScope();

    var index = new WordsIndex(
      myEvents.SelectMany(e => e.Name.Split(Separator))
        .ToHashSet()
        .Select((e, index) => (e, index)).ToDictionary(p => p.e, p => p.index)
    );

    var eventsWithTokens = myEvents
      .Select(e => new EventWithTokens(e, ConvertMessageToTokens(e.Name, index)))
      .Where(et => et.Tokens.Length <= maxTokensInEvent)
      .ToList();

    if (useGroupsAsEventNames)
    {
      foreach (var evt in eventsWithTokens)
      {
        evt.Event.Name = evt.Event.Group;
      }
    }
    else
    {
      var clusters = Dbscan.Dbscan.CalculateClusters(new EventsIndex(eventsWithTokens), 4, 2);

      ProcessClusters(clusters, index);
      ProcessUnclusteredEvents(clusters.UnclusteredObjects);
    }

    foreach (var @event in eventsWithTokens)
    {
      var bxesEvent = new InMemoryEventImpl(DateTime.UtcNow.Ticks, new BxesStringValue(@event.Event.Name), []);
      myWriter.HandleEvent(new BxesEventEvent<InMemoryEventImpl>(bxesEvent));
    }

    DisposeWriter();
  }

  private static void ProcessUnclusteredEvents(IReadOnlyList<EventWithTokens> events)
  {
    AnsiConsole.MarkupLine("[blue]UNCLUSTERED[/]");
    foreach (var evt in events)
    {
      evt.Event.Name = evt.Event.Group;
      Console.WriteLine(evt.Event.Name);
    }
  }

  private void DisposeWriter()
  {
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

  private static void ProcessClusters(ClusterSet<EventWithTokens> clusters, WordsIndex index)
  {
    foreach (var cluster in clusters.Clusters)
    {
      if (cluster.Objects.Count is 0) continue;

      var lcs = cluster.Objects.Skip(1)
        .Aggregate(cluster.Objects[0].Tokens, (current, obj) => ClusteringUtils.FindLcs(obj.Tokens, current).Lcs);

      AnsiConsole.MarkupLine("[blue]CLUSTER[/]");
      AnsiConsole.Markup("[blue]LCS:[/] ");

      foreach (var idx in lcs)
      {
        Console.Write($"{index.WordByToken(idx)} ");
      }

      AdjustEventsNames(cluster, lcs, index);

      AnsiConsole.WriteLine();

      foreach (var obj in cluster.Objects)
      {
        Console.WriteLine(obj.Event.Name);
      }

      AnsiConsole.WriteLine();
      AnsiConsole.WriteLine();
    }
  }

  private static void AdjustEventsNames(Cluster<EventWithTokens> cluster, int[] lcs, WordsIndex index)
  {
    foreach (var evt in cluster.Objects)
    {
      evt.Event.Name = CreateNewClusteredEventName(evt, lcs, index);
    }
  }

  private static string CreateNewClusteredEventName(EventWithTokens evt, int[] lcs, WordsIndex index)
  {
    var indices = ClusteringUtils.FindLcs(evt.Tokens, lcs).FirstIndices;

    var newMessage = new StringBuilder();
    newMessage.Append('[');

    var lcsIndex = 0;
    var addedPlaceholders = 0;
    for (var i = 0; i < evt.Tokens.Length; ++i)
    {
      if (lcsIndex >= indices.Count || i != indices[lcsIndex])
      {
        newMessage.Append($"({addedPlaceholders + 1})");
        ++addedPlaceholders;
      }
      else
      {
        newMessage.Append(index.WordByToken(evt.Tokens[i]));
        ++lcsIndex;
      }

      if (i < evt.Tokens.Length - 1)
      {
        newMessage.Append(' ');
      }
    }

    newMessage.Append(']');

    lcsIndex = 0;
    for (var i = 0; i < evt.Tokens.Length; ++i)
    {
      if (lcsIndex < indices.Count && i == indices[lcsIndex])
      {
        lcsIndex++;
        continue;
      }

      newMessage.Append($"{{{index.WordByToken(evt.Tokens[i])}}}");
    }

    return newMessage.ToString();
  }

  private static int[] ConvertMessageToTokens(string message, WordsIndex index) =>
    message.Split(Separator).Select(word => index[word]).ToArray();
}

internal static class IndexExtensions
{
  extension(WordsIndex index)
  {
    public string WordByToken(int i) => index.GetKeyAtIndex(index.IndexOfValue(i));
  }
}