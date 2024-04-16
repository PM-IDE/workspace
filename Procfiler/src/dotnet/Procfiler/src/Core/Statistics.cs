using Procfiler.Utils;

namespace Procfiler.Core;

internal struct Statistics()
{
  public int EventsCount { get; set; } = 0;
  public int EventsWithStackTraces { get; set; } = 0;
  public PercentValue EventsWithManagedThreadIs { get; } = new();
  public Dictionary<string, int> StackTracesPerEvents { get; } = new();
  public Dictionary<string, int> EventsCountMap { get; } = new();
  public Dictionary<string, List<string>> EventNamesToPayloadProperties { get; } = new();


  public void LogMyself(IProcfilerLogger logger)
  {
    var sb = new StringBuilder();
    sb.LogPrimitiveValue(nameof(EventsWithStackTraces), EventsWithStackTraces)
      .LogPrimitiveValue(nameof(EventsCount), EventsCount)
      .LogPrimitiveValue(nameof(EventsWithManagedThreadIs), EventsWithManagedThreadIs.Percent)
      .LogDictionary(nameof(StackTracesPerEvents), StackTracesPerEvents)
      .LogDictionary(nameof(EventsCountMap), EventsCountMap)
      .LogDictionary(nameof(EventNamesToPayloadProperties), EventNamesToPayloadProperties);

    logger.LogTrace("Statistics:\n{Value}\n", sb.ToString());
  }
}