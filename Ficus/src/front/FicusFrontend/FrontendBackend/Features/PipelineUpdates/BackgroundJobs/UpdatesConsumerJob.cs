using FrontendBackend.Features.PipelineUpdates.Kafka.PipelineUpdates;
using Google.Protobuf;

namespace FrontendBackend.Features.PipelineUpdates.BackgroundJobs;

public class UpdatesConsumerJob(IPipelinePartsUpdatesConsumer consumer, ILogger<UpdatesConsumerJob> logger) : IHostedService
{
  public Task StartAsync(CancellationToken cancellationToken)
  {
    Task.Factory.StartNew(() => ExecuteConsumerRoutine(cancellationToken), cancellationToken);
    return Task.CompletedTask;
  }

  public Task StopAsync(CancellationToken cancellationToken)
  {
    return Task.CompletedTask;
  }
  
  private void ExecuteConsumerRoutine(CancellationToken stoppingToken)
  {
    consumer.StartUpdatesConsuming(stoppingToken, update =>
    {
      logger.LogInformation("Update {Update}", update.ToByteString().ToString());
    });
  }
}