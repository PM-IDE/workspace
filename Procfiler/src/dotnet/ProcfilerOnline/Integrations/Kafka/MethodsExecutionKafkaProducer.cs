using Confluent.Kafka;
using Core.Container;
using Core.Events.EventRecord;

namespace ProcfilerOnline.Integrations.Kafka;

public class MethodsExecutionKafkaMessage
{
  public required string MethodFullName { get; init; }
  public required List<EventRecordWithMetadata> Events { get; init; }
}

[AppComponent]
public class MethodsExecutionKafkaProducer : IKafkaProducer<Guid, MethodsExecutionKafkaMessage>
{
  private readonly IProducer<Guid, MethodsExecutionKafkaMessage> myProducer =
    new ProducerBuilder<Guid, MethodsExecutionKafkaMessage>(new Config())
      .SetKeySerializer(GuidSerializer.Instance)
      .SetValueSerializer(JsonSerializer<MethodsExecutionKafkaMessage>.Instance)
      .Build();


  public void Produce(string topicName, Guid key, MethodsExecutionKafkaMessage value)
  {
    myProducer.Produce(topicName, new Message<Guid, MethodsExecutionKafkaMessage>
    {
      Key = key,
      Value = value,
    });
  }
}