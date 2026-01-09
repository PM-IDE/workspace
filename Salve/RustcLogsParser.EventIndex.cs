using Dbscan;

namespace Salve;

internal partial class RustcLogsParser
{
  private enum EventKind
  {
    Message,
    Method
  }

  private class Event(EventKind kind, string message, string group) : IPointData
  {
    public EventKind Kind => kind;
    public string Name { get; set; } = message;
    public string Group => group;
    public Point Point => default;
  }

  private record EventWithTokens(Event Event, int[] Tokens) : IPointData
  {
    public Point Point => Event.Point;
  }

  private class EventsIndex(List<EventWithTokens> events) : ISpatialIndex<PointInfo<EventWithTokens>>
  {
    private readonly Dictionary<string, List<PointInfo<EventWithTokens>>> myEventsByGroups =
      events
        .GroupBy(e => e.Event.Group)
        .ToDictionary(
          e => e.Key,
          e => e
            .Select(evt => new PointInfo<EventWithTokens>(evt))
            .ToList()
        );


    public IReadOnlyList<PointInfo<EventWithTokens>> Search() =>
      myEventsByGroups.Values.SelectMany(v => v).Where(e => ShouldCluster(e.Item.Event)).ToList();

    private static bool ShouldCluster(Event e) => e.Kind is EventKind.Message;

    public IReadOnlyList<PointInfo<EventWithTokens>> Search(in IPointData p, double epsilon)
    {
      var point = (PointInfo<EventWithTokens>)p;
      var result = new List<PointInfo<EventWithTokens>>();

      foreach (var evt in myEventsByGroups[point.Item.Event.Group])
      {
        if (!ShouldCluster(evt.Item.Event) || ReferenceEquals(evt, point))
        {
          continue;
        }

        if (point.Item.Tokens.Length != evt.Item.Tokens.Length) continue;

        var distance = ClusteringUtils.CalculateEditDistance(point.Item.Tokens, evt.Item.Tokens);

        if (distance <= epsilon)
        {
          result.Add(evt);
        }
      }

      return result;
    }
  }
}