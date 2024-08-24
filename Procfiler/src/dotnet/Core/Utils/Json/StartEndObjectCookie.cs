using System.Text.Json;

namespace Core.Utils.Json;

public readonly struct StartEndObjectCookie : IDisposable
{
  private readonly Utf8JsonWriter myWriter;


  public StartEndObjectCookie(Utf8JsonWriter writer, string? propertyName = null)
  {
    myWriter = writer;

    if (propertyName is { })
    {
      writer.WriteStartObject(propertyName);
      return;
    }

    writer.WriteStartObject();
  }


  public void Dispose()
  {
    myWriter.WriteEndObject();
  }
}

public readonly struct StartEndArrayCookie : IDisposable
{
  private readonly Utf8JsonWriter myWriter;


  public StartEndArrayCookie(Utf8JsonWriter writer, string propertyName)
  {
    myWriter = writer;
    writer.WriteStartArray(propertyName);
  }


  public void Dispose()
  {
    myWriter.WriteEndArray();
  }
}