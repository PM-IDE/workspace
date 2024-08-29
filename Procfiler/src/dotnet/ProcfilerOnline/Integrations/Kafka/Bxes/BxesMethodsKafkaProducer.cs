using Bxes.Kafka;
using Bxes.Models.Domain.Values;
using Bxes.Writer;
using Bxes.Writer.Stream;
using Confluent.Kafka;
using Core.Bxes;
using Core.Container;
using Core.Events.EventRecord;
using Microsoft.Extensions.Options;
using ProcfilerOnline.Core.Settings;

namespace ProcfilerOnline.Integrations.Kafka.Bxes;

public interface IBxesMethodsKafkaProducer : IKafkaProducer<Guid, BxesKafkaMethodsExecutionMessage>;

public class BxesKafkaMethodsExecutionMessage
{
  public required string MethodName { get; init; }
  public required List<EventRecordWithMetadata> Trace { get; init; }
}

[AppComponent]
public class BxesMethodsKafkaProducer(IOptions<OnlineProcfilerSettings> settings) : IBxesMethodsKafkaProducer
{
  private readonly IBxesStreamWriter myWriter = new BxesKafkaStreamWriter<BxesEvent>(
    BxesUtil.CreateSystemMetadata(),
    settings.Value.KafkaSettings.TopicName,
    new ProducerConfig
    {
      BootstrapServers = settings.Value.KafkaSettings.BootstrapServers
    });


  public void Produce(Guid key, BxesKafkaMethodsExecutionMessage value)
  {
    List<AttributeKeyValue> metadata =
    [
      new AttributeKeyValue(new BxesStringValue("MethodName"), new BxesStringValue(value.MethodName))
    ];

    myWriter.HandleEvent(new BxesTraceVariantStartEvent(1, metadata));

    foreach (var eventRecord in value.Trace)
    {
      myWriter.HandleEvent(new BxesEventEvent<BxesEvent>(new BxesEvent(eventRecord, true)));
    }

    myWriter.HandleEvent(BxesKafkaTraceVariantEndEvent.Instance);
  }
}