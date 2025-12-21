using System.Text.Json;

namespace Core.InstrumentalProfiler.DepsJson;

public static class JsonElementExtensions
{
  extension(JsonElement element)
  {
    public string GetPropertyStringValueOrThrow(string propertyName) =>
      element.TryGetPropertyStringValue(propertyName) ??
      throw new ArgumentOutOfRangeException(propertyName);

    public string? TryGetPropertyStringValue(string propertyName) =>
      element.TryGetProperty(propertyName, out var property) ? property.GetString() : null;

    public bool? TryGetPropertyBoolValue(string propertyName) =>
      element.TryGetProperty(propertyName, out var property) ? property.GetBoolean() : null;

    public string GetStringValueOrThrow() =>
      element.GetString() ?? throw new NullReferenceException();

    public JsonElement? GetPropertyOrNull(string propertyName) =>
      element.TryGetProperty(propertyName, out var property) ? property : null;
  }
}