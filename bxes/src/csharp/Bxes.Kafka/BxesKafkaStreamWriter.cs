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
  private readonly BxesWriteMetadata myWriteMetadata = new()
  {
    ValuesEnumerator = new LogValuesEnumerator([]),
    ValuesIndices = [],
    KeyValueIndices = []
  };

  private readonly IProducer<Guid, BxesKafkaTrace<TEvent>> myProducer;
  private readonly List<TEvent> myTraceEvents = [];
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
        myTraceEvents.Add(eventEvent.Event);
        break;
      case BxesTraceVariantStartEvent:
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
    if (myTraceEvents.Count == 0) return;

    try
    {
      myProducer.ProduceAsync(myTopicName, new Message<Guid, BxesKafkaTrace<TEvent>>
      {
        Key = Guid.NewGuid(),
        Value = new BxesKafkaTrace<TEvent>
        {
          Events = myTraceEvents
        }
      }).GetAwaiter().GetResult();
    }
    finally
    {
      myTraceEvents.Clear();
    }
  }

  public void Dispose()
  {
    myProducer.Dispose();
  }
}