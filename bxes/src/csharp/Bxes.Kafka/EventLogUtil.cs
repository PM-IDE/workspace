using Bxes.Models.Domain;
using Bxes.Writer.Stream;

namespace Bxes.Kafka;

public static class EventLogUtil
{
  public static IEnumerable<BxesStreamEvent> ToKafkaEventsStream(this IEventLog log)
  {
    foreach (var variant in log.Traces)
    {
      yield return new BxesTraceVariantStartEvent(1, []);

      foreach (var @event in variant.ToEventsStream())
      {
        yield return @event;
      }

      yield return BxesKafkaTraceVariantEndEvent.Instance;
    }
  }
}