using Bxes.Kafka;
using Bxes.Models.Domain;
using Bxes.Models.System;
using Bxes.Utils;
using Confluent.Kafka;
using FicusKafkaIntegration;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Configuration.EnvironmentVariables;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;

namespace IntegrationTests;


public class FicusKafkaProducerSettings
{
  public required string Topic { get; init; }
  public required string BootstrapServers { get; init; }
}

public class FicusKafkaIntegrationTests
{
  [Test]
  public void DoTest()
  {
    var configuration = new ConfigurationBuilder().Add(new EnvironmentVariablesConfigurationSource()).Build();
    var producerSettings = configuration.GetSection(nameof(FicusKafkaProducerSettings)).Get<FicusKafkaProducerSettings>()!;

    var eventLog = RandomLogsGenerator.CreateSimpleLog();
    var writer = CreateBxesKafkaWriter(producerSettings);
    foreach (var @event in eventLog.ToEventsStream())
    {
      writer.HandleEvent(@event);
    }

    var logger = LoggerFactory.Create(_ => { }).CreateLogger<PipelinePartsUpdatesConsumer>();
    var pipelinePartsConsumerSettings = configuration
      .GetSection(nameof(PipelinePartsUpdateKafkaSettings))
      .Get<PipelinePartsUpdateKafkaSettings>()!;

    var consumer = new PipelinePartsUpdatesConsumer(Options.Create(pipelinePartsConsumerSettings), logger);

    foreach (var update in consumer.StartUpdatesConsuming(CancellationToken.None))
    {
      Console.WriteLine(update.ProcessCaseMetadata);
    }
  }

  private BxesKafkaStreamWriter<IEvent> CreateBxesKafkaWriter(FicusKafkaProducerSettings settings) => new(
    new SystemMetadata(),
    settings.Topic,
    new ProducerConfig
    {
      BootstrapServers = settings.BootstrapServers
    }
  );
}