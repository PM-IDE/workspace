using Confluent.Kafka;
using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using ProcfilerOnline.Core.Settings;

namespace ProcfilerOnline.Integrations.Kafka;

public class MethodsExecutionKafkaMessage
{
  public required string MethodFullName { get; init; }
  public required List<EventRecordWithMetadata> Events { get; init; }
}

[AppComponent]
public class MethodsExecutionKafkaProducer(
  IOptions<OnlineProcfilerSettings> settings,
  IProcfilerLogger logger
) : IKafkaProducer<Guid, MethodsExecutionKafkaMessage>
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


  public void Produce(Guid key, MethodsExecutionKafkaMessage value)
  {
    try
    {
      var result = myProducer.ProduceAsync(settings.Value.KafkaSettings.TopicName, new Message<Guid, MethodsExecutionKafkaMessage>
      {
        Key = key,
        Value = value,
      }).GetAwaiter().GetResult();

      if (result.Status is not PersistenceStatus.Persisted)
      {
        logger.LogError("Failed to persist message in kafka, {Status}", result.Status);
      }
    }
    catch (Exception ex)
    {
      logger.LogError(ex, "Failed to send method execution message");
    }
  }
}