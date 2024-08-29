using Bxes.Models.Domain;
using Bxes.Models.System;
using Bxes.Writer;
using Bxes.Writer.Stream;
using Confluent.Kafka;

namespace Bxes.Kafka;

public class BxesKafkaTraceVariantEndEvent : BxesStreamEvent
{
  public static BxesKafkaTraceVariantEndEvent Instance { get; } = new();


  private BxesKafkaTraceVariantEndEvent()
  {
  }
}

public class BxesKafkaStreamWriter<TEvent> : IBxesStreamWriter where TEvent : IEvent
{
  private class CurrentTraceInfo
  {
    public List<TEvent> Events { get; } = [];
    public required IReadOnlyList<AttributeKeyValue> Metadata { get; init; }
  }

  private readonly ISystemMetadata mySystemMetadata;
  private readonly IProducer<Guid, BxesKafkaTrace<TEvent>> myProducer;
  private CurrentTraceInfo myTraceInfo = null!;
  private readonly string myTopicName;


  public BxesKafkaStreamWriter(ISystemMetadata systemMetadata, string topicName, ProducerConfig producerConfig)
  {
    myTopicName = topicName;
    mySystemMetadata = systemMetadata;

    var writeMetadata = new BxesWriteMetadata
    {
      ValuesEnumerator = new LogValuesEnumerator(systemMetadata.ValueAttributeDescriptors),
      ValuesIndices = [],
      KeyValueIndices = []
    };

    myProducer = new ProducerBuilder<Guid, BxesKafkaTrace<TEvent>>(producerConfig)
      .SetKeySerializer(GuidSerializer.Instance)
      .SetValueSerializer(new BxesKafkaEventSerializer<TEvent>(writeMetadata))
      .Build();
  }


  public void HandleEvent(BxesStreamEvent @event)
  {
    switch (@event)
    {
      case BxesKafkaTraceVariantEndEvent:
        ProduceTrace();
        break;
      case BxesEventEvent<TEvent> eventEvent:
        myTraceInfo.Events.Add(eventEvent.Event);
        break;
      case BxesTraceVariantStartEvent startEvent:
      {
        myTraceInfo = new CurrentTraceInfo
        {
          Metadata = startEvent.Metadata.AsReadOnly()
        };

        break;
      }
      case BxesKeyValueEvent:
      case BxesLogMetadataClassifierEvent:
      case BxesLogMetadataExtensionEvent:
      case BxesLogMetadataGlobalEvent:
      case BxesLogMetadataPropertyEvent:
      case BxesRecalculateIndicesEvent:
      case BxesValueEvent:
        break;
      default:
        throw new ArgumentOutOfRangeException(nameof(@event));
    }
  }

  private void ProduceTrace()
  {
    if (myTraceInfo.Events.Count == 0) return;

    myProducer.ProduceAsync(myTopicName, new Message<Guid, BxesKafkaTrace<TEvent>>
    {
      Key = Guid.NewGuid(),
      Value = new BxesKafkaTrace<TEvent>
      {
        SystemMetadata = mySystemMetadata,
        Metadata = myTraceInfo.Metadata,
        Events = myTraceInfo.Events
      }
    }).GetAwaiter().GetResult();
  }

  public void Dispose()
  {
    myProducer.Dispose();
  }
}