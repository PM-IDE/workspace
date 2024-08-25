using Confluent.Kafka;

namespace Bxes.Kafka;

public class GuidSerializer : ISerializer<Guid>, IDeserializer<Guid>
{
  public static GuidSerializer Instance { get; } = new();


  private GuidSerializer()
  {
  }


  public byte[] Serialize(Guid data, SerializationContext context) => data.ToByteArray();
  public Guid Deserialize(ReadOnlySpan<byte> data, bool isNull, SerializationContext context) => new(data);
}