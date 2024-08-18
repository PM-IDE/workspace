using System.Text.Json;
using Confluent.Kafka;

namespace ProcfilerOnline.Integrations.Kafka;

public class JsonSerializer<T> : ISerializer<T>
{
  public static JsonSerializer<T> Instance { get; } = new();


  private JsonSerializer()
  {
  }


  public byte[] Serialize(T data, SerializationContext context) => JsonSerializer.SerializeToUtf8Bytes(data);
}