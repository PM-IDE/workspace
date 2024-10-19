using Confluent.Kafka;
using Ficus;
using FrontendBackend.Features.PipelineUpdates.Settings;
using FrontendBackend.Utils;
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

    logger.WaitUntilTopicExists(settings.Value.BootstrapServers, settings.Value.Topic);
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
}