using Confluent.Kafka;
using Core.Container;
using Core.Events.EventRecord;
using Microsoft.Extensions.Options;
using ProcfilerOnline.Core.Settings;

namespace ProcfilerOnline.Integrations.Kafka;

public class MethodsExecutionKafkaMessage
{
  public required string MethodFullName { get; init; }
  public required List<EventRecordWithMetadata> Events { get; init; }
}

[AppComponent]
public class MethodsExecutionKafkaProducer(IOptions<OnlineProcfilerSettings> settings) : IKafkaProducer<Guid, MethodsExecutionKafkaMessage>
{
  private readonly IProducer<Guid, MethodsExecutionKafkaMessage> myProducer =
    new ProducerBuilder<Guid, MethodsExecutionKafkaMessage>(new ProducerConfig
      {
        BootstrapServers = settings.Value.KafkaSettings.BootstrapServers,
        Acks = Acks.All
      })
      .SetKeySerializer(GuidSerializer.Instance)
      .SetValueSerializer(JsonSerializer<MethodsExecutionKafkaMessage>.Instance)
      .Build();


  public void Produce(string topicName, Guid key, MethodsExecutionKafkaMessage value)
  {
    myProducer.ProduceAsync(topicName, new Message<Guid, MethodsExecutionKafkaMessage>
    {
      Key = key,
      Value = value,
    }).GetAwaiter().GetResult();
  }
}