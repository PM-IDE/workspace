using Ficus;

namespace IntegrationTests;

public static class GrpcRequestsCreator
{
  public static GrpcSubscribeToKafkaRequest CreateSubscribeToKafkaRequest(
    FicusIntegrationTestsSettings settings, string? subscriptionName = null)
  {
    return new GrpcSubscribeToKafkaRequest
    {
      SubscriptionMetadata = new GrpcKafkaSubscriptionMetadata
      {
        SubscriptionName = subscriptionName ?? "My subscription"
      },
      ConnectionMetadata = new GrpcKafkaConnectionMetadata
      {
        TopicName = settings.ConsumerTopic,
        Metadata =
        {
          new GrpcKafkaMetadata
          {
            Key = "bootstrap.servers",
            Value = settings.ConsumerBootstrapServers
          },
          new GrpcKafkaMetadata
          {
            Key = "group.id",
            Value = settings.ConsumerGroup
          },
          new GrpcKafkaMetadata
          {
            Key = "auto.offset.reset",
            Value = "earliest"
          }
        }
      }
    };
  }
  
  public static GrpcAddPipelineRequest CreateAddGetNamesLogPipelineRequest(
    GrpcGuid subscriptionId, FicusIntegrationTestsSettings settings)
  {
    return new GrpcAddPipelineRequest
    {
      PipelineRequest = new GrpcKafkaPipelineExecutionRequest
      {
        SubscriptionId = subscriptionId,
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
        TopicName = settings.ProducerTopic,
        Metadata =
        {
          new GrpcKafkaMetadata
          {
            Key = "bootstrap.servers",
            Value = settings.ProducerBootstrapServers
          }
        }
      }
    };
  }
}