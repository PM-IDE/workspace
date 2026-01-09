using System.Text;
using System.Text.RegularExpressions;
using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Writer.Stream;
using Dbscan;
using Spectre.Console;
using WordsIndex = System.Collections.Generic.SortedList<string, int>;

namespace Salve;

internal partial class RustcLogsParser(string outputPath, bool useGroupsAsEventNames) : ILogsProcessor
{
  private const char Separator = ' ';

  [GeneratedRegex("rustc(_[a-z]+)+(::[a-z_]+)*")]
  private static partial Regex MessageGroupRegex();

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
    line = MsRegex().Replace(line, string.Empty).Trim();

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

    if (!line.StartsWith("INFO") && !line.StartsWith("DEBUG")) return false;
    if (MessageGroupRegex().Match(line) is not { } match) return false;

    messageGroup = match.ValueSpan;

    return true;
  }

  public void Dispose()
  {
    using var _ = myLock.EnterScope();

    var index = new WordsIndex(
      myEvents.SelectMany(e => e.Message.Split(Separator))
        .ToHashSet()
        .Select((e, index) => (e, index)).ToDictionary(p => p.e, p => p.index)
    );

    var eventsWithTokens = myEvents
      .Select(e => new EventWithTokens(e, ConvertMessageToTokens(e.Message, index)))
      .Where(et => et.Tokens.Length < 8)
      .ToList();

    if (useGroupsAsEventNames)
    {
      foreach (var evt in eventsWithTokens)
      {
        evt.Event.Message = evt.Event.Group;
      }
    }
    else
    {
      var clusters = Dbscan.Dbscan.CalculateClusters(new EventsIndex(eventsWithTokens), 4, 2);

      ProcessClusters(clusters, index);
      LogUnclusteredEvents(clusters);
    }

    foreach (var @event in eventsWithTokens)
    {
      var bxesEvent = new InMemoryEventImpl(DateTime.UtcNow.Ticks, new BxesStringValue(@event.Event.Message), []);
      myWriter.HandleEvent(new BxesEventEvent<InMemoryEventImpl>(bxesEvent));
    }

    DisposeWriter();
  }

  private static void LogUnclusteredEvents(ClusterSet<EventWithTokens> clusters)
  {
    AnsiConsole.MarkupLine("[blue]UNCLUSTERED[/]");
    foreach (var obj in clusters.UnclusteredObjects)
    {
      obj.Event.Message = $"[{obj.Event.Message}]";
      Console.WriteLine(obj.Event.Message);
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
        Console.WriteLine(obj.Event.Message);
      }

      AnsiConsole.WriteLine();
      AnsiConsole.WriteLine();
    }
  }

  private static void AdjustEventsNames(Cluster<EventWithTokens> cluster, int[] lcs, WordsIndex index)
  {
    foreach (var evt in cluster.Objects)
    {
      evt.Event.Message = CreateNewEventName(evt, lcs, index);
    }
  }

  private static string CreateNewEventName(EventWithTokens evt, int[] lcs, WordsIndex index)
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