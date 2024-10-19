using Bxes.Models.Domain;
using Bxes.Models.System;
using Bxes.Writer;

namespace Bxes.Kafka;

public class BxesKafkaTrace<TEvent> where TEvent : IEvent
{
  public required IReadOnlyList<AttributeKeyValue> Metadata { get; init; }
  public required List<TEvent> Events { get; init; }
  public required ISystemMetadata SystemMetadata { get; init; }
}