using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Models.System;
using Bxes.Reader;
using Bxes.Utils;
using Bxes.Writer;

namespace Bxes.Kafka;

public class BxesKafkaConsumer
{
  public class ConsumedBxesTrace
  {
    public required List<AttributeKeyValue> Metadata { get; init; }
    public required List<IEvent> Events { get; init; }
  }


  private readonly BxesReadMetadata myMetadata = new()
  {
    Values = [],
    KeyValues = []
  };


  public ConsumedBxesTrace Consume(byte[] rawBytes)
  {
    var ms = new MemoryStream(rawBytes);
    var reader = new BinaryReader(ms);
    var readContext = new BxesReadContext(reader, myMetadata, new SystemMetadata());

    BxesReadUtils.ReadSystemMetadata(readContext);
    BxesReadUtils.ReadValues(readContext);
    BxesReadUtils.ReadKeyValuePairs(readContext);

    var traceMetadata = BxesReadUtils.ReadTraceVariantMetadata(readContext);
    var events = BxesReadUtils.ReadTraceVariantEvents(readContext);

    return new ConsumedBxesTrace
    {
      Metadata = traceMetadata,
      Events = events
    };
  }
}