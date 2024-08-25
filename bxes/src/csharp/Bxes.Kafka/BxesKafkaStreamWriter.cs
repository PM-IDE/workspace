using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Writer;
using Bxes.Writer.Stream;
using Confluent.Kafka;

namespace Bxes.Kafka;


public class BxesKafkaStreamWriter<TEvent>(string topicName) : IBxesStreamWriter where TEvent : IEvent
{
  private readonly IProducer<Guid, BxesKafkaEvent> myProducer;

  private readonly List<TEvent> myTraceEvents = [];
  private readonly BxesWriteMetadata myWriteMetadata = new()
  {
    ValuesEnumerator = new LogValuesEnumerator([]),
    ValuesIndices = [],
    KeyValueIndices = []
  };


  public void HandleEvent(BxesStreamEvent @event)
  {
    switch (@event)
    {
      case BxesTraceVariantStartEvent:
        ProduceTrace();
        break;
      case BxesEventEvent<TEvent> eventEvent:
        myTraceEvents.Add(eventEvent.Event);
        break;
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
    try
    {
      var metadata = UpdateWriteMetadata();

      myProducer.ProduceAsync(topicName, new Message<Guid, BxesKafkaEvent>
      {
        Key = Guid.NewGuid(),
        Value = new BxesKafkaEvent
        {
          Trace = CreateKafkaTrace(),
          KafkaMetadataUpdate = metadata
        }
      }).GetAwaiter().GetResult();
    }
    finally
    {
      myTraceEvents.Clear();
    }
  }

  private BxesKafkaMetadataUpdate UpdateWriteMetadata()
  {
    var newValues = new List<BxesValue>();
    var newKeyValues = new List<(uint, uint)>();

    foreach (var @event in myTraceEvents)
    {
      foreach (var value in myWriteMetadata.ValuesEnumerator.EnumerateEventValues(@event))
      {
        if (myWriteMetadata.ValuesIndices.ContainsKey(value)) continue;

        myWriteMetadata.ValuesIndices[value] = (uint)myWriteMetadata.ValuesIndices.Count;
        newValues.Add(value);
      }

      foreach (var keyValue in myWriteMetadata.ValuesEnumerator.EnumerateEventKeyValuePairs(@event))
      {
        if (myWriteMetadata.KeyValueIndices.ContainsKey(keyValue)) continue;

        var keyIndex = myWriteMetadata.ValuesIndices[keyValue.Key];
        var valueIndex = myWriteMetadata.ValuesIndices[keyValue.Value];

        newKeyValues.Add((keyIndex, valueIndex));
      }
    }

    return new BxesKafkaMetadataUpdate
    {
      NewValues = newValues,
      NewKeyValues = newKeyValues
    };
  }

  private BxesKafkaTrace CreateKafkaTrace() => new()
  {
    Events = myTraceEvents.Select(e => new BxesKafkaTraceEvent
    {
      NameIndex = myWriteMetadata.ValuesIndices[new BxesStringValue(e.Name)],
      TimeStamp = e.Timestamp,
      Attributes = e.Attributes.Select(attr => myWriteMetadata.KeyValueIndices[attr]).ToList(),
    }).ToList()
  };


  public void Dispose()
  {
    myProducer.Dispose();
  }
}