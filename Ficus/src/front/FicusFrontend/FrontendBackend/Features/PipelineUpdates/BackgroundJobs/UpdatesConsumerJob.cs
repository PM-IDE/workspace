﻿using FrontendBackend.Features.PipelineUpdates.Kafka.PipelineUpdates;
using FrontendBackend.Features.PipelineUpdates.Services;

namespace FrontendBackend.Features.PipelineUpdates.BackgroundJobs;

public class UpdatesConsumerJob(
  IPipelinePartsUpdatesConsumer consumer,
  ILogger<UpdatesConsumerJob> logger,
  IPipelinePartsUpdatesRepository repository
) : IHostedService
{
  public Task StartAsync(CancellationToken cancellationToken)
  {
    Task.Factory.StartNew(async () => await ExecuteConsumerRoutine(cancellationToken), cancellationToken);
    return Task.CompletedTask;
  }

  public Task StopAsync(CancellationToken cancellationToken)
  {
    return Task.CompletedTask;
  }
  
  private async Task ExecuteConsumerRoutine(CancellationToken stoppingToken)
  {
    try
    {
      logger.LogInformation("Starting the pipeline context values updates consuming routine");
      foreach (var update in consumer.StartUpdatesConsuming(stoppingToken))
      {
        logger.LogInformation("Consumed an update from kafka: {Update}", update.ToString());
        await repository.ProcessUpdate(update);

        logger.LogInformation("Processed the update");
      }
    }
    catch (Exception ex)
    {
      logger.LogError(ex, "Error while consuming updates");
    }
  }
}