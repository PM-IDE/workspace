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

    var valuesCount = reader.ReadUInt32();
    for (var i = 0; i < valuesCount; ++i)
    {
      myMetadata.Values.Add(BxesValue.Parse(reader, myMetadata.Values));
    }

    var keyValuesCount = reader.ReadUInt32();
    for (var i = 0; i < keyValuesCount; ++i)
    {
      myMetadata.KeyValues.Add(new KeyValuePair<uint, uint>((uint)reader.ReadLeb128Unsigned(), (uint)reader.ReadLeb128Unsigned()));
    }

    var metadataCount = reader.ReadUInt32();
    var traceMetadata = new List<AttributeKeyValue>();

    for (var i = 0; i < metadataCount; ++i)
    {
      var kvIndices = myMetadata.KeyValues[(int)reader.ReadLeb128Unsigned()];
      var key = (BxesStringValue)myMetadata.Values[(int)kvIndices.Key];
      var value = myMetadata.Values[(int)kvIndices.Value];

      traceMetadata.Add(new AttributeKeyValue(key, value));
    }

    var eventsCount = reader.ReadUInt32();
    var events = new List<IEvent>((int)eventsCount);

    for (var i = 0; i < eventsCount; ++i)
    {
      events.Add(BxesReadUtils.ReadEvent(new BxesReadContext(reader, myMetadata, SystemMetadata.Default)));
    }

    return new ConsumedBxesTrace
    {
      Metadata = traceMetadata,
      Events = events
    };
  }
}