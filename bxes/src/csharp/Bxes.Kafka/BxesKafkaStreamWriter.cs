using Bxes.Models.Domain;
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

  private readonly BxesWriteMetadata myWriteMetadata = new()
  {
    ValuesEnumerator = new LogValuesEnumerator([]),
    ValuesIndices = [],
    KeyValueIndices = []
  };

  private readonly IProducer<Guid, BxesKafkaTrace<TEvent>> myProducer;
  private CurrentTraceInfo myTraceInfo = null!;
  private readonly string myTopicName;


  public BxesKafkaStreamWriter(string topicName, ProducerConfig producerConfig)
  {
    myTopicName = topicName;

    myProducer = new ProducerBuilder<Guid, BxesKafkaTrace<TEvent>>(producerConfig)
      .SetKeySerializer(GuidSerializer.Instance)
      .SetValueSerializer(new BxesKafkaEventSerializer<TEvent>(myWriteMetadata))
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