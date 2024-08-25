using System.Text.Json;
using Confluent.Kafka;

namespace Bxes.Kafka;

public class JsonSerializer<T> : ISerializer<T>, IDeserializer<T>
{
  public static JsonSerializer<T> Instance { get; } = new();


  private JsonSerializer()
  {
  }


  public byte[] Serialize(T data, SerializationContext context) => JsonSerializer.SerializeToUtf8Bytes(data);
  public T Deserialize(ReadOnlySpan<byte> data, bool isNull, SerializationContext context) => JsonSerializer.Deserialize<T>(data);
}