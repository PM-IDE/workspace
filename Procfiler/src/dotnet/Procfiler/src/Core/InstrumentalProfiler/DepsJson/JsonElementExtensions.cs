namespace Procfiler.Core.InstrumentalProfiler.DepsJson;

public static class JsonElementExtensions
{
  public static string GetPropertyStringValueOrThrow(this JsonElement element, string propertyName) =>
    element.TryGetPropertyStringValue(propertyName) ??
    throw new ArgumentOutOfRangeException(propertyName);

  public static string? TryGetPropertyStringValue(this JsonElement element, string propertyName) =>
    element.TryGetProperty(propertyName, out var property) ? property.GetString() : null;

  public static bool? TryGetPropertyBoolValue(this JsonElement element, string propertyName) =>
    element.TryGetProperty(propertyName, out var property) ? property.GetBoolean() : null;

  public static string GetStringValueOrThrow(this JsonElement element) =>
    element.GetString() ?? throw new NullReferenceException();

  public static JsonElement? GetPropertyOrNull(this JsonElement element, string propertyName) =>
    element.TryGetProperty(propertyName, out var property) ? property : null;
}