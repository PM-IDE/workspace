using Confluent.Kafka;
using Ficus;

namespace FrontendBackend.Features.PipelineUpdates.Kafka.PipelineUpdates;

class GrpcKafkaUpdateDeserializer : IDeserializer<GrpcKafkaUpdate>
{
  public static GrpcKafkaUpdateDeserializer Instance { get; } = new();


  private GrpcKafkaUpdateDeserializer()
  {
  }


  public GrpcKafkaUpdate Deserialize(ReadOnlySpan<byte> data, bool isNull, SerializationContext context)
  {
    return GrpcKafkaUpdate.Parser.ParseFrom(data);
  }
}