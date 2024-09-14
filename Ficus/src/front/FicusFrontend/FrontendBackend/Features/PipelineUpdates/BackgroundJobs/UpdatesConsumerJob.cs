using FrontendBackend.Features.PipelineUpdates.Kafka.PipelineUpdates;

namespace FrontendBackend.Features.PipelineUpdates.BackgroundJobs;

public class UpdatesConsumerJob(IPipelinePartsUpdatesConsumer consumer, ILogger<UpdatesConsumerJob> logger) : BackgroundService
{
  protected override Task ExecuteAsync(CancellationToken stoppingToken)
  {
    consumer.StartUpdatesConsuming(stoppingToken, _ =>
    {
      logger.LogInformation("Update");
    });

    return Task.CompletedTask;
  }
}