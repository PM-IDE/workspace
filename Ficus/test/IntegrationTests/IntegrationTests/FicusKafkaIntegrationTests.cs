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
  private IConfiguration myConfiguration;


  [SetUp]
  public void InitConfiguration()
  {
    myConfiguration = new ConfigurationBuilder().Add(new EnvironmentVariablesConfigurationSource()).Build();
  }
  
  [Test]
  public void EventNamesTest()
  {
    var eventLog = GenerateTestEventLog();
    
    ProduceEventLogToKafka(eventLog);

    AssertNamesLogMatchesOriginal(eventLog, ConsumeAllUpdates());
  }

  private void AssertNamesLogMatchesOriginal(IEventLog eventLog, IReadOnlyList<GrpcKafkaUpdate> updates)
  {
    Assert.That(eventLog.Traces, Has.Count.EqualTo(updates.Count));
    
    var lastNameLog = updates.Last().ContextValues.First(c => c.Value.ContextValueCase is GrpcContextValue.ContextValueOneofCase.NamesLog);
    foreach (var (trace, grpcTrace) in eventLog.Traces.Zip(lastNameLog.Value.NamesLog.Log.Traces))
    {
      Assert.That(grpcTrace.Events, Has.Count.EqualTo(trace.Events.Count));
      foreach (var (traceEvent, grpcEventName) in trace.Events.Zip(grpcTrace.Events))
      {
        Assert.That(grpcEventName, Is.EqualTo(traceEvent.Name));
      }
    }
  }

  private static IEventLog GenerateTestEventLog()
  {
    var eventLog = GenerateRandomEventLog();
    SetEventLogMetadata(eventLog);

    return eventLog;
  }

  private static IEventLog GenerateRandomEventLog() => RandomLogsGenerator.CreateSimpleLog(new RandomLogGenerationParameters
  {
    EventsCount = new LowerUpperBound(1, 10),
    VariantsCount = new LowerUpperBound(1, 10)
  });

  private static void SetEventLogMetadata(IEventLog eventLog)
  {
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
  }

  private void ProduceEventLogToKafka(IEventLog eventLog)
  {
    var writer = CreateBxesKafkaWriter();
    foreach (var @event in eventLog.ToKafkaEventsStream())
    {
      writer.HandleEvent(@event);
    }
    
    Thread.Sleep(10_000);
  }

  private IReadOnlyList<GrpcKafkaUpdate> ConsumeAllUpdates()
  {
    var logger = LoggerFactory.Create(_ => { }).CreateLogger<PipelinePartsUpdatesConsumer>();
    var pipelinePartsConsumerSettings = myConfiguration
      .GetSection(nameof(PipelinePartsUpdateKafkaSettings))
      .Get<PipelinePartsUpdateKafkaSettings>()!;

    return ConsumeAllUpdates(pipelinePartsConsumerSettings, logger);
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

  private BxesKafkaStreamWriter<IEvent> CreateBxesKafkaWriter()
  {
    var settings = myConfiguration.GetSection(nameof(FicusKafkaProducerSettings)).Get<FicusKafkaProducerSettings>()!;

    return new BxesKafkaStreamWriter<IEvent>(
      new SystemMetadata(),
      settings.Topic,
      new ProducerConfig
      {
        BootstrapServers = settings.BootstrapServers
      }
    );
  }
}