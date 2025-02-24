using Confluent.Kafka;

namespace FicusKafkaIntegration;

internal class GuidDeserializer : IDeserializer<Guid>
{
  public static GuidDeserializer Instance { get; } = new();


  private GuidDeserializer()
  {
  }


  public Guid Deserialize(ReadOnlySpan<byte> data, bool isNull, SerializationContext context) => new(data);
}