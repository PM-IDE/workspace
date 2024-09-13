using Confluent.Kafka;
using Ficus;
using Microsoft.Extensions.Options;

namespace FrontendBackend.Integrations.Kafka;

public class PipelinePartsUpdateKafkaSettings
{
  public string Topic { get; set; }
  public string BootstrapServiers { get; set; }
}

public class PipelinePartsUpdatesConsumer(IOptions<PipelinePartsUpdateKafkaSettings> settings)
{
  private readonly IConsumer<Guid, GrpcKafkaUpdate> myConsumer =
    new ConsumerBuilder<Guid, GrpcKafkaUpdate>(new ConsumerConfig
      {
        BootstrapServers = settings.Value.BootstrapServiers,
        GroupId = nameof(PipelinePartsUpdatesConsumer)
      })
      .SetKeyDeserializer(GuidDeserializer.Instance)
      .SetValueDeserializer(GrpcKafkaUpdateDeserializer.Instance)
      .Build();


  public void StartUpdatesConsuming()
  {
    myConsumer.Subscribe(settings.Value.Topic);

    try
    {
      while (true)
      {
        var result = myConsumer.Consume();

        myConsumer.Commit(result);
      }
    }
    finally
    {
      myConsumer.Close();
    }
  }
}