using Bxes.Models.Domain;

namespace Bxes.Kafka;

public class BxesKafkaTrace<TEvent> where TEvent : IEvent
{
  public required List<TEvent> Events { get; init; }
}
