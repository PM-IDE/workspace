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
  public required string ProcessName { get; init; }
  public required string CaseName { get; init; }
  public required List<EventRecordWithMetadata> Trace { get; init; }

  public ExtendedMethodInfo? MethodInfo { get; init; }
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


  public void Produce(Guid key, BxesKafkaMethodsExecutionMessage message)
  {
    List<AttributeKeyValue> metadata =
    [
      new(new BxesStringValue("case_name"), new BxesStringValue(message.CaseName)),
      new(new BxesStringValue("process_name"), new BxesStringValue(message.ProcessName))
    ];

    if (message.MethodInfo is { } methodInfo)
    {
      metadata.AddRange(
      [
        new AttributeKeyValue(new BxesStringValue("method_name"), new BxesStringValue(methodInfo.Name)),
        new AttributeKeyValue(new BxesStringValue("method_signature"), new BxesStringValue(methodInfo.Signature))
      ]);
    }

    myWriter.HandleEvent(new BxesTraceVariantStartEvent(1, metadata));

    foreach (var eventRecord in message.Trace)
    {
      myWriter.HandleEvent(new BxesEventEvent<BxesEvent>(new BxesEvent(eventRecord, true)));
    }

    myWriter.HandleEvent(BxesKafkaTraceVariantEndEvent.Instance);
  }
}