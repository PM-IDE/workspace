using Core.Events.EventRecord;
using Core.Events.EventsCollection;
using Core.Utils;
using Procfiler.Core.EventRecord;

namespace Procfiler.Commands.CollectClrEvents.Split;

public static class SplitEventsHelper
{
  public static Func<EventRecordWithMetadata, long> ManagedThreadIdExtractor { get; } =
    @event => @event.ManagedThreadId;

  public static Func<EventRecordWithMetadata, string> EventClassKeyExtractor { get; } =
    @event => @event.EventClass.Replace("/", "_");

  public static Dictionary<TKey, IEventsCollection> SplitByKey<TKey>(
    IProcfilerLogger logger,
    IEventsCollection events,
    Func<EventRecordWithMetadata, TKey> keyExtractor) where TKey : notnull
  {
    var map = new Dictionary<TKey, List<EventRecordWithMetadata>>();
    var initialListCapacity = (int)events.Count / 20;
    foreach (var (_, eventRecord) in events)
    {
      var list = map.GetOrCreate(keyExtractor(eventRecord), () => new List<EventRecordWithMetadata>(initialListCapacity));
      list.Add(eventRecord);
    }

    return map.ToDictionary(
      pair => pair.Key,
      pair => (IEventsCollection)new EventsCollectionImpl(pair.Value.ToArray(), logger)
    );
  }
}