using Confluent.Kafka;
using Ficus;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;

namespace FicusKafkaIntegration;

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
    const string ConsumerGroupId = $"{nameof(PipelinePartsUpdatesConsumer)}::{nameof(StartUpdatesConsuming)}";
    var consumer = PipelinePartsResultsConsumptionUtil.CreateConsumerAndWaitUntilTopicExists(settings.Value, ConsumerGroupId, logger);

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
        if (result.IsPartitionEOF) continue;
        
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

public static class PipelinePartsResultsConsumptionUtil
{
  public static IConsumer<Guid, GrpcKafkaUpdate> CreateConsumerAndWaitUntilTopicExists(
    PipelinePartsUpdateKafkaSettings settings,
    string consumerGroupId,
    ILogger logger)
  {
    var config = new ConsumerConfig
    {
      BootstrapServers = settings.BootstrapServers,
      GroupId = consumerGroupId,
      EnablePartitionEof = true,
      AutoOffsetReset = AutoOffsetReset.Earliest
    };

    var consumer = new ConsumerBuilder<Guid, GrpcKafkaUpdate>(config)
      .SetKeyDeserializer(GuidDeserializer.Instance)
      .SetValueDeserializer(GrpcKafkaUpdateDeserializer.Instance)
      .Build();

    logger.WaitUntilTopicExists(settings.BootstrapServers, settings.Topic);
    consumer.Subscribe(settings.Topic);

    return consumer;
  }
}