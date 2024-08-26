using Bxes.Models.Domain;
using Bxes.Models.System;
using Bxes.Reader;
using Bxes.Utils;

namespace Bxes.Kafka;

public class BxesKafkaConsumer
{
  private readonly BxesReadMetadata myMetadata = new()
  {
    Values = [],
    KeyValues = []
  };


  public List<IEvent> Consume(byte[] rawBytes)
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

    var eventsCount = reader.ReadUInt32();
    var events = new List<IEvent>((int)eventsCount);

    for (var i = 0; i < eventsCount; ++i)
    {
      events.Add(BxesReadUtils.ReadEvent(new BxesReadContext(reader, myMetadata, SystemMetadata.Default)));
    }

    return events;
  }
}