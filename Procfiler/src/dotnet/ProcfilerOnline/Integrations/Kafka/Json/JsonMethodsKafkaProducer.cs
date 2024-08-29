using Confluent.Kafka;
using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using ProcfilerOnline.Core.Settings;

namespace ProcfilerOnline.Integrations.Kafka.Json;

public class JsonMethodsExecutionKafkaMessage
{
  public required string MethodFullName { get; init; }
  public required List<JsonEventRecordWithMetadataKafkaDto> Events { get; init; }
}

public class JsonEventRecordWithMetadataKafkaDto
{
  public required long ManagedThreadId { get; init; }
  public required string EventClass { get; init; }
  public required string EventName { get; init; }
  public required EventRecordTime Time { get; init; }
  public required int StackTraceId { get; init; }
  public required Dictionary<string, string> Attributes { get; init; }


  public static JsonEventRecordWithMetadataKafkaDto FromEventRecord(EventRecordWithMetadata eventRecord) => new()
  {
    Attributes = eventRecord.Metadata.ToDictionary(),
    Time = eventRecord.Time,
    StackTraceId = eventRecord.StackTraceId,
    EventClass = eventRecord.EventClass,
    EventName = eventRecord.EventName,
    ManagedThreadId = eventRecord.ManagedThreadId
  };
}

public interface IJsonMethodsKafkaProducer : IKafkaProducer<Guid, JsonMethodsExecutionKafkaMessage>;

[AppComponent]
public class JsonMethodsKafkaProducer(
  IOptions<OnlineProcfilerSettings> settings,
  IProcfilerLogger logger
) : IJsonMethodsKafkaProducer
{
  private readonly Lazy<IProducer<Guid, JsonMethodsExecutionKafkaMessage>> myProducer = new(() =>
    new ProducerBuilder<Guid, JsonMethodsExecutionKafkaMessage>(
        new ProducerConfig
        {
          BootstrapServers = settings.Value.KafkaSettings.BootstrapServers,
          Acks = Acks.All
        }
      )
      .SetKeySerializer(GuidSerializer.Instance)
      .SetValueSerializer(JsonSerializer<JsonMethodsExecutionKafkaMessage>.Instance)
      .Build()
  );


  public void Produce(Guid key, JsonMethodsExecutionKafkaMessage value)
  {
    try
    {
      var topicName = settings.Value.KafkaSettings.TopicName;
      var message = new Message<Guid, JsonMethodsExecutionKafkaMessage>
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