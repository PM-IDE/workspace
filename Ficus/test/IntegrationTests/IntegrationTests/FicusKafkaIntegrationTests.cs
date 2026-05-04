using Bxes.Kafka;
using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Models.System;
using Bxes.Utils;
using Bxes.Writer;
using Confluent.Kafka;
using Ficus;
using FicusKafkaConstants;
using FicusKafkaIntegration;
using Grpc.Core.Utils;
using IntegrationTests.Base;
using Microsoft.Extensions.Logging;

namespace IntegrationTests;

[TestFixture]
[FixtureLifeCycle(LifeCycle.SingleInstance)]
public class FicusKafkaIntegrationTests : TestWithFicusBackendOneKafkaSubscription
{
  [Test]
  public void EventNamesTest()
  {
    var eventLog = GenerateTestEventLog();
    ProduceEventLogToKafka(eventLog);
    AssertNamesLogMatchesOriginal(eventLog, ConsumeAllUpdates(eventLog.Traces.Count));
  }

  [Test]
  public void SameTraceIdTest()
  {
    var eventLog = GenerateTestEventLog();
    var newSameTraceId = Guid.NewGuid();

    foreach (var variant in eventLog.Traces)
    {
      variant.Metadata.Remove(variant.Metadata.FirstOrDefault(m => m.Key.Value == FicusKafkaKeys.CaseId)!);
      variant.Metadata.Add(new AttributeKeyValue(new BxesStringValue(FicusKafkaKeys.CaseId), new BxesGuidValue(newSameTraceId)));
    }

    ProduceEventLogToKafka(eventLog);
    AssertNamesLogMatchesMergedOriginal(eventLog, ConsumeAllUpdates(eventLog.Traces.Count));
  }

  private void AssertNamesLogMatchesMergedOriginal(IEventLog eventLog, IReadOnlyList<GrpcKafkaUpdate> updates)
  {
    var namesLog = FindLastNamesLog(updates);

    //for not taking that last trace, as log remains from the previous test, todo: fix this after
    var lastTrace = namesLog.Value.NamesLog.Log.Traces.Last();

    Assert.That(lastTrace.Events, Has.Count.EqualTo(eventLog.Traces.Select(t => t.Events.Count).Sum()));
    foreach (var (traceEvent, grpcEventName) in eventLog.Traces.SelectMany(t => t.Events).Zip(lastTrace.Events))
    {
      Assert.That(grpcEventName, Is.EqualTo(traceEvent.Name));
    }
  }

  private void AssertNamesLogMatchesOriginal(IEventLog eventLog, IReadOnlyList<GrpcKafkaUpdate> updates)
  {
    Assert.That(eventLog.Traces, Has.Count.EqualTo(updates.Count));

    var lastNameLog = FindLastNamesLog(updates);
    foreach (var (trace, grpcTrace) in eventLog.Traces.Zip(lastNameLog.Value.NamesLog.Log.Traces))
    {
      Assert.That(grpcTrace.Events, Has.Count.EqualTo(trace.Events.Count));
      foreach (var (traceEvent, grpcEventName) in trace.Events.Zip(grpcTrace.Events))
      {
        Assert.That(grpcEventName, Is.EqualTo(traceEvent.Name));
      }
    }
  }

  private GrpcContextValueWithKeyName FindLastNamesLog(IReadOnlyList<GrpcKafkaUpdate> updates)
  {
    var lastUpdate = updates.Last();
    var stream = KafkaClient.GetCurrentContextValues(new GrpcGetCurrentContextValuesRequest
    {
      PipelineId = lastUpdate.ProcessCaseMetadata.PipelineId,
      ProcessName = lastUpdate.ProcessCaseMetadata.ProcessName,
      SubscriptionId = lastUpdate.ProcessCaseMetadata.SubscriptionId
    });

    var contextValues = stream.ResponseStream.ToListAsync().GetAwaiter().GetResult();

    return contextValues.Where(c => c.ResultCase == GrpcPipelinePartExecutionResult.ResultOneofCase.PipelinePartResult)
      .SelectMany(c => c.PipelinePartResult.ContextValues)
      .First(c => c.Value.ContextValueCase is GrpcContextValue.ContextValueOneofCase.NamesLog);
  }

  private static IEventLog GenerateTestEventLog()
  {
    var eventLog = GenerateRandomEventLog();
    SetEventLogMetadata(eventLog);

    return eventLog;
  }

  private static IEventLog GenerateRandomEventLog() => RandomLogsGenerator.CreateSimpleLog(new RandomLogGenerationParameters
  {
    EventsCount = new LowerUpperBound(1, 5),
    VariantsCount = new LowerUpperBound(5, 10),
    EventAttributesCount = new LowerUpperBound(1, 5)
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
        new AttributeKeyValue(new BxesStringValue(FicusKafkaKeys.CaseDisplayNameKey), new BxesStringValue(CaseName)),
        new AttributeKeyValue(new BxesStringValue(FicusKafkaKeys.CaseNameParts), new BxesStringValue(CaseName)),
        new AttributeKeyValue(new BxesStringValue(FicusKafkaKeys.ProcessNameKey), new BxesStringValue(ProcessName)),
        new AttributeKeyValue(new BxesStringValue(FicusKafkaKeys.CaseId), new BxesGuidValue(Guid.NewGuid()))
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

  private IReadOnlyList<GrpcKafkaUpdate> ConsumeAllUpdates(int updatesCount)
  {
    var logger = LoggerFactory.Create(_ => { }).CreateLogger<PipelinePartsUpdatesConsumer>();
    return ConsumeAllUpdates(logger, updatesCount);
  }

  private IReadOnlyList<GrpcKafkaUpdate> ConsumeAllUpdates(ILogger logger, int updatesCount)
  {
    const string ConsumerGroupId = $"{nameof(FicusKafkaIntegrationTests)}::{nameof(ConsumeAllUpdates)}";
    var consumer =
      PipelinePartsResultsConsumptionUtil.CreateConsumerAndWaitUntilTopicExists(PipelinePartsSettings, ConsumerGroupId, logger);

    List<GrpcKafkaUpdate> result = [];
    var consumed = 0;

    while (true)
    {
      var consumeResult = consumer.Consume();
      if (consumeResult is null || consumeResult.IsPartitionEOF) break;

      result.Add(consumeResult.Message.Value);
      consumer.Commit();

      if (++consumed == updatesCount)
      {
        break;
      }
    }

    return result;
  }

  private BxesKafkaStreamWriter<IEvent> CreateBxesKafkaWriter() =>
    new(
      new SystemMetadata(),
      ProducerSettings.Topic,
      new ProducerConfig
      {
        BootstrapServers = ProducerSettings.BootstrapServers
      }
    );
}