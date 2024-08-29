using Bxes.Models.Domain;
using Bxes.Writer;
using Confluent.Kafka;

namespace Bxes.Kafka;

public class BxesKafkaEventSerializer<TEvent>(BxesWriteMetadata writeMetadata)
  : ISerializer<BxesKafkaTrace<TEvent>> where TEvent : IEvent
{
  public byte[] Serialize(BxesKafkaTrace<TEvent> data, SerializationContext context)
  {
    using var stream = new MemoryStream();
    using var writer = new BinaryWriter(stream);

    var writeContext = new BxesWriteContext(writer, writeMetadata);
    BxesWriteUtils.WriteValuesAttributesDescriptors(data.SystemMetadata.ValueAttributeDescriptors, writeContext);

    WriteCollectionAndCount(writer, data, BxesWriteUtils.WriteEventValues, (ctx, trace) =>
    {
      var count = 0;
      foreach (var attribute in trace.Metadata)
      {
        if (BxesWriteUtils.WriteValueIfNeeded(attribute.Key, ctx)) ++count;
        if (BxesWriteUtils.WriteValueIfNeeded(attribute.Value, ctx)) ++count;
      }

      return count;
    });

    WriteCollectionAndCount(writer, data, BxesWriteUtils.WriteEventKeyValues, (ctx, trace) =>
    {
      var count = 0;

      foreach (var attribute in trace.Metadata)
      {
        if (BxesWriteUtils.WriteKeyValuePairIfNeeded(attribute, ctx)) ++count;
      }

      return count;
    });

    writer.Write((uint)data.Metadata.Count);
    foreach (var attribute in data.Metadata)
    {
      BxesWriteUtils.WriteKeyValueIndex(attribute, writeContext);
    }

    writer.Write((uint)data.Events.Count);
    foreach (var @event in data.Events)
    {
      BxesWriteUtils.WriteEvent(@event, writeContext);
    }

    return stream.GetBuffer();
  }

  private void WriteCollectionAndCount(
    BinaryWriter writer,
    BxesKafkaTrace<TEvent> trace,
    Func<TEvent, BxesWriteContext, int> eventWriteFunc,
    Func<BxesWriteContext, BxesKafkaTrace<TEvent>, int> metadataWriteFunc)
  {
    var valuesCountPos = writer.BaseStream.Position;
    writer.Write((uint)0);

    var count = 0;

    var writeContext = new BxesWriteContext(writer, writeMetadata);

    count += metadataWriteFunc(writeContext, trace);

    foreach (var @event in trace.Events)
    {
      count += eventWriteFunc(@event, writeContext);
    }

    BxesWriteUtils.WriteCount(writer, valuesCountPos, (uint)count);
  }
}