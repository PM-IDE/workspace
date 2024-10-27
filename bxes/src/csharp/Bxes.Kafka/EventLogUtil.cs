using Bxes.Models.Domain;
using Bxes.Writer.Stream;

namespace Bxes.Kafka;

public static class EventLogUtil
{
  public static IEnumerable<BxesStreamEvent> ToKafkaEventsStream(this IEventLog log)
  {
    foreach (var variant in log.Traces)
    {
      foreach (var @event in variant.ToKafkaEventsStream())
      {
        yield return @event;
      }
    }
  }

  public static IEnumerable<BxesStreamEvent> ToKafkaEventsStream(this ITraceVariant variant)
  {
    yield return new BxesTraceVariantStartEvent(1, variant.Metadata);

    foreach (var @event in variant.ToEventsStream())
    {
      yield return @event;
    }

    yield return BxesKafkaTraceVariantEndEvent.Instance;
  }
}