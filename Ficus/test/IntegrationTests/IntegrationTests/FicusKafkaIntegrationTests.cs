using Bxes.Kafka;
using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Models.System;
using Bxes.Utils;
using Bxes.Writer;
using Confluent.Kafka;
using Ficus;
using FicusKafkaIntegration;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Configuration.EnvironmentVariables;
using Microsoft.Extensions.Logging;

namespace IntegrationTests;


public class FicusKafkaProducerSettings
{
  public required string Topic { get; init; }
  public required string BootstrapServers { get; init; }
}

public class FicusKafkaIntegrationTests
{
  [Test]
  public void EventNamesTest()
  {
    var configuration = new ConfigurationBuilder().Add(new EnvironmentVariablesConfigurationSource()).Build();
    var producerSettings = configuration.GetSection(nameof(FicusKafkaProducerSettings)).Get<FicusKafkaProducerSettings>()!;

    var eventLog = RandomLogsGenerator.CreateSimpleLog(new RandomLogGenerationParameters
    {
      EventsCount = new LowerUpperBound(1, 10),
      VariantsCount = new LowerUpperBound(1, 10)
    });

    var writer = CreateBxesKafkaWriter(producerSettings);

    const string ProcessName = nameof(ProcessName);
    const string CaseName = nameof(CaseName);

    foreach (var variant in eventLog.Traces)
    {
      variant.Metadata.Clear();
      variant.Metadata.AddRange(
      [
        new AttributeKeyValue(new BxesStringValue("case_name"), new BxesStringValue(CaseName)),
        new AttributeKeyValue(new BxesStringValue("process_name"), new BxesStringValue(ProcessName))
      ]);
    }

    foreach (var @event in eventLog.ToKafkaEventsStream())
    {
      writer.HandleEvent(@event);
    }

    var logger = LoggerFactory.Create(_ => { }).CreateLogger<PipelinePartsUpdatesConsumer>();
    var pipelinePartsConsumerSettings = configuration
      .GetSection(nameof(PipelinePartsUpdateKafkaSettings))
      .Get<PipelinePartsUpdateKafkaSettings>()!;

    Thread.Sleep(10_000);
    var updates = ConsumeAllUpdates(pipelinePartsConsumerSettings, logger);

    var lastNameLog = updates.Last().ContextValues.First(c => c.Value.ContextValueCase is GrpcContextValue.ContextValueOneofCase.NamesLog);
    foreach (var (trace, grpcTrace) in eventLog.Traces.Zip(lastNameLog.Value.NamesLog.Log.Traces))
    {
      Assert.That(grpcTrace.Events.Count, Is.EqualTo(trace.Events.Count));
      foreach (var (traceEvent, grpcEventName) in trace.Events.Zip(grpcTrace.Events))
      {
        Assert.That(grpcEventName, Is.EqualTo(traceEvent.Name));
      }
    }
  }

  private static IReadOnlyList<GrpcKafkaUpdate> ConsumeAllUpdates(PipelinePartsUpdateKafkaSettings settings, ILogger logger)
  {
    const string ConsumerGroupId = $"{nameof(FicusKafkaIntegrationTests)}::{nameof(ConsumeAllUpdates)}";
    var consumer = PipelinePartsResultsConsumptionUtil.CreateConsumerAndWaitUntilTopicExists(settings, ConsumerGroupId, logger);

    List<GrpcKafkaUpdate> result = [];
    while (true)
    {
      var consumeResult = consumer.Consume();
      if (consumeResult.IsPartitionEOF) break;
      
      result.Add(consumeResult.Message.Value);
      consumer.Commit();
    }

    return result;
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