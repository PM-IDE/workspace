using Confluent.Kafka;

namespace ProcfilerOnline.Integrations.Kafka;

public class GuidSerializer : ISerializer<Guid>
{
  public static GuidSerializer Instance { get; } = new();


  private GuidSerializer()
  {
  }


  public byte[] Serialize(Guid data, SerializationContext context) => data.ToByteArray();
}