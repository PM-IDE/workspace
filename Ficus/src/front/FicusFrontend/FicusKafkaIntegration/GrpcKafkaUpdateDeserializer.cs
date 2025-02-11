using Confluent.Kafka;
using Ficus;

namespace FicusKafkaIntegration;

internal class GrpcKafkaUpdateDeserializer : IDeserializer<GrpcKafkaUpdate>
{
  public static GrpcKafkaUpdateDeserializer Instance { get; } = new();


  private GrpcKafkaUpdateDeserializer()
  {
  }


  public GrpcKafkaUpdate Deserialize(ReadOnlySpan<byte> data, bool isNull, SerializationContext context) =>
    GrpcKafkaUpdate.Parser.ParseFrom(data);
}