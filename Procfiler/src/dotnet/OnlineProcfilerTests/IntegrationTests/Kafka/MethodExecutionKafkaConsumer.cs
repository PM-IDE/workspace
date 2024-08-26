using Confluent.Kafka;
using ProcfilerOnline.Core.Settings;
using ProcfilerOnline.Integrations.Kafka;
using ProcfilerOnline.Integrations.Kafka.Json;

namespace OnlineProcfilerTests.IntegrationTests.Kafka;

public class MethodExecutionKafkaConsumer : IDisposable
{
  private readonly IConsumer<Guid, JsonMethodsExecutionKafkaMessage> myConsumer;


  public MethodExecutionKafkaConsumer(OnlineProcfilerSettings settings)
  {
    myConsumer = new ConsumerBuilder<Guid, JsonMethodsExecutionKafkaMessage>(
        new ConsumerConfig
        {
          BootstrapServers = settings.KafkaSettings.BootstrapServers,
          GroupId = "xd",
          EnablePartitionEof = true,
          AutoOffsetReset = AutoOffsetReset.Earliest,
        }
      )
      .SetKeyDeserializer(GuidSerializer.Instance)
      .SetValueDeserializer(JsonSerializer<JsonMethodsExecutionKafkaMessage>.Instance)
      .Build();

    myConsumer.Subscribe(settings.KafkaSettings.TopicName);
  }


  public List<JsonMethodsExecutionKafkaMessage> ConsumeAllEvents()
  {
    var messages = new List<JsonMethodsExecutionKafkaMessage>();
    while (true)
    {
      var result = myConsumer.Consume();
      if (result.IsPartitionEOF) break;

      messages.Add(result.Message.Value);
    }

    return messages;
  }

  public void Dispose()
  {
    myConsumer.Dispose();
  }
}