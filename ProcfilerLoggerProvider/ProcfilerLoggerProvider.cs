using System.Collections.Concurrent;
using System.Diagnostics.Tracing;
using Microsoft.Extensions.Logging;

namespace ProcfilerLoggerProvider;

public class ProcfilerLoggerProvider(LogLevel logLevel) : ILoggerProvider
{
  private readonly ConcurrentDictionary<string, ProcfilerLogger> myLoggers = [];


  public ILogger CreateLogger(string categoryName) => myLoggers.GetOrAdd(categoryName, _ => new ProcfilerLogger(logLevel));


  public void Dispose()
  {
    myLoggers.Clear();
  }
}

[EventSource(Name = $"{nameof(ProcfilerBusinessEventsSource)}")]
internal sealed class ProcfilerBusinessEventsSource : EventSource
{
  public const int ProcfilerBusinessEventId = 6000;

  
  public static ProcfilerBusinessEventsSource Instance { get; } = new();


  [Event(ProcfilerBusinessEventId, Level = EventLevel.LogAlways)]
  public void WriteBusinessEvent(LogLevel level, EventId eventId)
  {
    WriteEvent(ProcfilerBusinessEventId, (int)level, eventId.Id, eventId.Name);
  }
}

internal class ProcfilerLogger(LogLevel logLevel) : ILogger
{
  public void Log<TState>(LogLevel level, EventId eventId, TState state, Exception? exception, Func<TState, Exception?, string> formatter)
  {
    ProcfilerBusinessEventsSource.Instance.WriteBusinessEvent(level, eventId);
  }

  public bool IsEnabled(LogLevel level)
  {
    if (level == LogLevel.None) return false;
    
    return level < logLevel;
  }

  public IDisposable? BeginScope<TState>(TState state) where TState : notnull => default;
}