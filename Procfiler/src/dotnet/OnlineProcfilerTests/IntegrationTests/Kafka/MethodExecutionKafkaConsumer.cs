using Bxes.Kafka;
using Bxes.Models.Domain;
using Confluent.Kafka;
using ProcfilerOnline.Core.Settings;
using GuidSerializer = ProcfilerOnline.Integrations.Kafka.GuidSerializer;

namespace OnlineProcfilerTests.IntegrationTests.Kafka;

public class MethodExecutionKafkaConsumer : IDisposable
{
  private readonly IConsumer<Guid, byte[]> myConsumer;
  private readonly BxesKafkaConsumer myBxesKafkaConsumer;


  public MethodExecutionKafkaConsumer(OnlineProcfilerSettings settings)
  {
    myBxesKafkaConsumer = new BxesKafkaConsumer();
    myConsumer = new ConsumerBuilder<Guid, byte[]>(
        new ConsumerConfig
        {
          BootstrapServers = settings.KafkaSettings.BootstrapServers,
          GroupId = "xd",
          EnablePartitionEof = true,
          AutoOffsetReset = AutoOffsetReset.Earliest,
        }
      )
      .SetKeyDeserializer(GuidSerializer.Instance)
      .Build();

    myConsumer.Subscribe(settings.KafkaSettings.TopicName);
  }


  public List<List<IEvent>> ConsumeAllEvents()
  {
    var messages = new List<List<IEvent>>();
    while (true)
    {
      var result = myConsumer.Consume();
      if (result.IsPartitionEOF) break;

      messages.Add(myBxesKafkaConsumer.Consume(result.Message.Value));
    }

    return messages;
  }

  public void Dispose()
  {
    myConsumer.Dispose();
  }
}