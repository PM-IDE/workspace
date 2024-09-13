using Confluent.Kafka;
using Ficus;

namespace FrontendBackend.Integrations.Kafka;

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