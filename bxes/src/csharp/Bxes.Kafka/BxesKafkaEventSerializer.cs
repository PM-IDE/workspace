using Bxes.Models.Domain;
using Bxes.Writer;
using Confluent.Kafka;

namespace Bxes.Kafka;

public class BxesKafkaEventSerializer<TEvent>(BxesWriteMetadata writeMetadata) : ISerializer<BxesKafkaTrace<TEvent>> where TEvent : IEvent
{
  public byte[] Serialize(BxesKafkaTrace<TEvent> data, SerializationContext context)
  {
    using var stream = new MemoryStream();
    using var writer = new BinaryWriter(stream);

    WriteCollectionAndCount(writer, data.Events, BxesWriteUtils.WriteEventValues);
    WriteCollectionAndCount(writer, data.Events, BxesWriteUtils.WriteEventKeyValues);

    writer.Write((uint)data.Events.Count);
    foreach (var @event in data.Events)
    {
      BxesWriteUtils.WriteEvent(@event, new BxesWriteContext(writer, writeMetadata));
    }

    return stream.GetBuffer();
  }

  private void WriteCollectionAndCount(BinaryWriter writer, List<TEvent> events, Func<TEvent, BxesWriteContext, int> writeFunc)
  {
    var valuesCountPos = writer.BaseStream.Position;
    writer.Write((uint)0);

    var count = 0;
    foreach (var @event in events)
    {
      var valuesContext = new BxesWriteContext(writer, writeMetadata);
      count += writeFunc(@event, valuesContext);
    }

    BxesWriteUtils.WriteCount(writer, valuesCountPos, (uint)count);
  }
}