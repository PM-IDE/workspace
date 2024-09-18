using Confluent.Kafka;
using Ficus;
using FrontendBackend.Features.PipelineUpdates.Settings;
using Microsoft.Extensions.Options;

namespace FrontendBackend.Features.PipelineUpdates.Kafka.PipelineUpdates;

public interface IPipelinePartsUpdatesConsumer
{
  IEnumerable<GrpcKafkaUpdate> StartUpdatesConsuming(CancellationToken cancellationToken);
}

public class PipelinePartsUpdatesConsumer(
  IOptions<PipelinePartsUpdateKafkaSettings> settings, 
  ILogger<PipelinePartsUpdatesConsumer> logger
) : IPipelinePartsUpdatesConsumer
{
  public IEnumerable<GrpcKafkaUpdate> StartUpdatesConsuming(CancellationToken cancellationToken)
  {
    var config = new ConsumerConfig
    {
      BootstrapServers = settings.Value.BootstrapServers,
      GroupId = $"{nameof(PipelinePartsUpdatesConsumer)}::{nameof(StartUpdatesConsuming)}"
    };

    var consumer = new ConsumerBuilder<Guid, GrpcKafkaUpdate>(config)
      .SetKeyDeserializer(GuidDeserializer.Instance)
      .SetValueDeserializer(GrpcKafkaUpdateDeserializer.Instance)
      .Build();

    WaitUntilTopicExists(logger, settings.Value.BootstrapServers, settings.Value.Topic);
    consumer.Subscribe(settings.Value.Topic);

    try
    {
      if (cancellationToken.IsCancellationRequested)
      {
        logger.LogInformation("Cancellation is requested, stopping consumer routine");
        yield break;
      }

      while (true)
      {
        logger.LogInformation("Waiting for the next message from kafka");

        var result = consumer.Consume(cancellationToken);

        yield return result.Message.Value;

        consumer.Commit(result);
      }
    }
    finally
    {
      logger.LogInformation("Finishing pipeline parts context values updates consumer routine");
      consumer.Close();
    }
  }

  private static void WaitUntilTopicExists(ILogger logger, string bootstrapServers, string topicName)
  {
    var config = new AdminClientConfig
    {
      BootstrapServers = bootstrapServers
    };

    using var client = new AdminClientBuilder(config).Build();

    try
    {
      while (!client.GetMetadata(TimeSpan.FromSeconds(5)).Topics.Select(t => t.Topic).ToHashSet().Contains(topicName))
      {
        logger.LogInformation("The topic is not created, will wait");
        Thread.Sleep(TimeSpan.FromSeconds(1));
      }
    }
    catch (Exception ex)
    {
      logger.LogError(ex, "Failed to get metadata");
    }
  }
}