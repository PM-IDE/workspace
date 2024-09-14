using Confluent.Kafka;
using Ficus;
using FrontendBackend.Features.PipelineUpdates.Settings;
using Microsoft.Extensions.Options;

namespace FrontendBackend.Features.PipelineUpdates.Kafka.PipelineUpdates;

public interface IPipelinePartsUpdatesConsumer
{
  void StartUpdatesConsuming(CancellationToken cancellationToken, Action<GrpcKafkaUpdate> updatesHandler);
}

public class PipelinePartsUpdatesConsumer(
  IOptions<PipelinePartsUpdateKafkaSettings> settings, 
  ILogger<PipelinePartsUpdatesConsumer> logger
) : IPipelinePartsUpdatesConsumer
{
  public void StartUpdatesConsuming(CancellationToken cancellationToken, Action<GrpcKafkaUpdate> updatesHandler)
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

    consumer.Subscribe(settings.Value.Topic);

    try
    {
      if (cancellationToken.IsCancellationRequested)
      {
        return;
      }

      while (true)
      {
        var result = consumer.Consume(cancellationToken);

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