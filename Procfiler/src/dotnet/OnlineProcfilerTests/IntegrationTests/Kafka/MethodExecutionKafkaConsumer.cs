using Confluent.Kafka;
using ProcfilerOnline.Integrations.Kafka;

namespace OnlineProcfilerTests.IntegrationTests.Kafka;

public class MethodExecutionKafkaConsumer : IDisposable
{
  private readonly IConsumer<Guid, MethodsExecutionKafkaMessage> myConsumer;


  public MethodExecutionKafkaConsumer(string topicName)
  {
    myConsumer = new ConsumerBuilder<Guid, MethodsExecutionKafkaMessage>(
        new ConsumerConfig
        {
          BootstrapServers = "localhost:9092",
          GroupId = "xd",
          EnablePartitionEof = true,
          AutoOffsetReset = AutoOffsetReset.Latest,
        }
      )
      .SetKeyDeserializer(GuidSerializer.Instance)
      .SetValueDeserializer(JsonSerializer<MethodsExecutionKafkaMessage>.Instance)
      .Build();

    myConsumer.Subscribe(topicName);
  }


  public List<MethodsExecutionKafkaMessage> ConsumeAllEvents()
  {
    var messages = new List<MethodsExecutionKafkaMessage>();
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