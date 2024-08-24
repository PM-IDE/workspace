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
  public required List<EventRecordWithMetadataKafkaDto> Events { get; init; }
}

public class EventRecordWithMetadataKafkaDto
{
  public required long ManagedThreadId { get; init; }
  public required string EventClass { get; init; }
  public required string EventName { get; init; }
  public required EventRecordTime Time { get; init; }
  public required int StackTraceId { get; init; }
  public required Dictionary<string, string> Attributes { get; init; }


  public static EventRecordWithMetadataKafkaDto FromEventRecord(EventRecordWithMetadata eventRecord) => new()
  {
    Attributes = eventRecord.Metadata.ToDictionary(),
    Time = eventRecord.Time,
    StackTraceId = eventRecord.StackTraceId,
    EventClass = eventRecord.EventClass,
    EventName = eventRecord.EventName,
    ManagedThreadId = eventRecord.ManagedThreadId
  };
}

[AppComponent]
public class MethodsExecutionKafkaProducer(
  IOptions<OnlineProcfilerSettings> settings,
  IProcfilerLogger logger
) : IKafkaProducer<Guid, MethodsExecutionKafkaMessage>
{
  private readonly Lazy<IProducer<Guid, MethodsExecutionKafkaMessage>> myProducer = new(() =>
    new ProducerBuilder<Guid, MethodsExecutionKafkaMessage>(
      new ProducerConfig
      {
        BootstrapServers = settings.Value.KafkaSettings.BootstrapServers,
        Acks = Acks.All
      }
    )
    .SetKeySerializer(GuidSerializer.Instance)
    .SetValueSerializer(JsonSerializer<MethodsExecutionKafkaMessage>.Instance)
    .Build()
  );


  public void Produce(Guid key, MethodsExecutionKafkaMessage value)
  {
    try
    {
      var topicName = settings.Value.KafkaSettings.TopicName;
      var message = new Message<Guid, MethodsExecutionKafkaMessage>
      {
        Key = key,
        Value = value,
      };

      var result = myProducer.Value.ProduceAsync(topicName, message).GetAwaiter().GetResult();

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