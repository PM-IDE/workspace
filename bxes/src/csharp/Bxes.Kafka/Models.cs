using Bxes.Models.Domain;

namespace Bxes.Kafka;

public class BxesKafkaTrace
{
  public required List<BxesKafkaTraceEvent> Events { get; init; }
}

public class BxesKafkaTraceEvent
{
  public required uint NameIndex { get; init; }
  public required long TimeStamp { get; init; }
  public required List<uint> Attributes { get; init; }
}

public class BxesKafkaMetadataUpdate
{
  public required List<BxesValue> NewValues { get; init; }
  public required List<(uint, uint)> NewKeyValues { get; init; }
}

public class BxesKafkaEvent
{
  public required BxesKafkaMetadataUpdate KafkaMetadataUpdate { get; init; }
  public required BxesKafkaTrace Trace { get; init; }
}
