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

[TestFixture]
public class FicusKafkaIntegrationTests : TestWithFicusBackendBase
{
  [Test]
  public void EventNamesTest()
  {
    var subscriptionId = CreateFicusKafkaSubscription();

    try
    {
      var eventLog = GenerateTestEventLog();
      ProduceEventLogToKafka(eventLog);
      AssertNamesLogMatchesOriginal(eventLog, ConsumeAllUpdates());
    }
    finally
    {
      KafkaClient.UnsubscribeFromKafkaTopic(new GrpcUnsubscribeFromKafkaRequest
      {
        SubscriptionId = subscriptionId
      });
    }
  }

  private GrpcGuid CreateFicusKafkaSubscription()
  {
    var subscriptionResult = KafkaClient.SubscribeForKafkaTopic(new GrpcSubscribeToKafkaRequest
    {
      SubscriptionMetadata = new GrpcKafkaSubscriptionMetadata
      {
        SubscriptionName = "My subscription"
      },
      ConnectionMetadata = new GrpcKafkaConnectionMetadata
      {
        TopicName = TestsSettings.ConsumerTopic,
        Metadata =
        {
          new GrpcKafkaConsumerMetadata
          {
            Key = "bootstrap.servers",
            Value = TestsSettings.ConsumerBootstrapServers
          },
          new GrpcKafkaConsumerMetadata
          {
            Key = "group.id",
            Value = TestsSettings.ConsumerGroup
          },
          new GrpcKafkaConsumerMetadata
          {
            Key = "auto.offset.reset",
            Value = "earliest"
          }
        }
      }
    });
    
    Assert.That(subscriptionResult.ResultCase, Is.EqualTo(GrpcKafkaResult.ResultOneofCase.Success));

    var pipelineAdditionResult = KafkaClient.AddPipelineToSubscription(new GrpcAddPipelineRequest
    {
      PipelineRequest = new GrpcKafkaPipelineExecutionRequest
      {
        SubscriptionId = subscriptionResult.Success.Id,
        PipelineMetadata = new GrpcPipelineMetadata
        {
          Name = "TestPipeline"
        },
        PipelineRequest = new GrpcPipelineExecutionRequest
        {
          Pipeline = new GrpcPipeline
          {
            Parts =
            {
              new GrpcPipelinePartBase
              {
                ComplexContextRequestPart = new GrpcComplexContextRequestPipelinePart
                {
                  Keys = { new GrpcContextKey { Name = "names_event_log" } },
                  FrontendPartUuid = new GrpcUuid { Uuid = Guid.NewGuid().ToString() },
                  FrontendPipelinePartName = "PrintEventLog",
                  BeforePipelinePart = new GrpcPipelinePart
                  {
                    Configuration = new GrpcPipelinePartConfiguration(),
                    Name = "GetNamesEventLog"
                  }
                }
              }
            }
          }
        }
      },
      ProducerKafkaMetadata = new GrpcKafkaConnectionMetadata
      {
        TopicName = TestsSettings.ProducerTopic,
        Metadata =
        {
          new GrpcKafkaConsumerMetadata
          {
            Key = "bootstrap.servers",
            Value = TestsSettings.ProducerBootstrapServers
          }
        }
      }
    });

    Assert.That(pipelineAdditionResult.ResultCase, Is.EqualTo(GrpcKafkaResult.ResultOneofCase.Success));

    return subscriptionResult.Success.Id;
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
    return ConsumeAllUpdates(logger);
  }

  private IReadOnlyList<GrpcKafkaUpdate> ConsumeAllUpdates(ILogger logger)
  {
    const string ConsumerGroupId = $"{nameof(FicusKafkaIntegrationTests)}::{nameof(ConsumeAllUpdates)}";
    var consumer = PipelinePartsResultsConsumptionUtil.CreateConsumerAndWaitUntilTopicExists(PipelinePartsSettings, ConsumerGroupId, logger);

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
    var settings = Configuration.GetSection(nameof(FicusKafkaProducerSettings)).Get<FicusKafkaProducerSettings>()!;

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