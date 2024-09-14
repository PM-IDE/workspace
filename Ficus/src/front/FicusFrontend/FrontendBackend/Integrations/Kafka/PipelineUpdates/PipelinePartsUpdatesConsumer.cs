using Confluent.Kafka;
using Ficus;
using Microsoft.Extensions.Options;

namespace FrontendBackend.Integrations.Kafka.PipelineUpdates;

public class PipelinePartsUpdateKafkaSettings
{
  public string Topic { get; set; }
  public string BootstrapServiers { get; set; }
}

public interface IPipelinePartsUpdatesConsumer
{
  void StartUpdatesConsuming(Action<GrpcKafkaUpdate> updatesHandler);
}

public class PipelinePartsUpdatesConsumer(
  IOptions<PipelinePartsUpdateKafkaSettings> settings, 
  ILogger<PipelinePartsUpdatesConsumer> logger
) : IPipelinePartsUpdatesConsumer
{
  public void StartUpdatesConsuming(Action<GrpcKafkaUpdate> updatesHandler)
  {
    var config = new ConsumerConfig
    {
      BootstrapServers = settings.Value.BootstrapServiers,
      GroupId = $"{nameof(PipelinePartsUpdatesConsumer)}::{nameof(StartUpdatesConsuming)}"
    };

    var consumer = new ConsumerBuilder<Guid, GrpcKafkaUpdate>(config)
      .SetKeyDeserializer(GuidDeserializer.Instance)
      .SetValueDeserializer(GrpcKafkaUpdateDeserializer.Instance)
      .Build();

    consumer.Subscribe(settings.Value.Topic);

    try
    {
      while (true)
      {
        var result = consumer.Consume();

        try
        {
          updatesHandler(result.Message.Value);
        }
        catch (Exception ex)
        {
          logger.LogError(ex, "Failed to handle new message, will commit offset anyway");
        }
        finally
        {
          consumer.Commit(result);
        }
      }
    }
    finally
    {
      consumer.Close();
    }
  }
}