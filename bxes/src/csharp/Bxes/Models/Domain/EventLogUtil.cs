using Bxes.Writer.Stream;

namespace Bxes.Models.Domain;

public static class EventLogUtil
{
  public static IEnumerable<BxesStreamEvent> ToEventsStream(this IEventLog log)
  {
    foreach (var @event in log.Metadata.ToEventsStream())
    {
      yield return @event;
    }

    foreach (var @event in log.ToTracesEventStream())
    {
      yield return @event;
    }
  }

  public static IEnumerable<BxesStreamEvent> ToTracesEventStream(this IEventLog log)
  {
    foreach (var variant in log.Traces)
    {
      yield return new BxesTraceVariantStartEvent(variant.Count, variant.Metadata);

      foreach (var @event in variant.ToEventsStream())
      {
        yield return @event;
      }
    }
  }

  public static IEnumerable<BxesStreamEvent> ToEventsStream(this ITraceVariant variant)
  {
    foreach (var @event in variant.Events)
    {
      yield return new BxesEventEvent<IEvent>(@event);
    }
  }

  public static bool EqualsRegardingOrder<T>(IList<T> firstList, IList<T> secondList)
  {
    if (firstList.Count != secondList.Count) return false;

    var firstSet = firstList.ToHashSet();
    var secondSet = secondList.ToHashSet();

    foreach (var first in firstSet)
    {
      if (!secondSet.Contains(first)) return false;
    }

    foreach (var second in secondSet)
    {
      if (!firstSet.Contains(second)) return false;
    }

    return true;
  }
}