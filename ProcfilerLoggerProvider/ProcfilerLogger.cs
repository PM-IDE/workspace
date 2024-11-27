using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;

namespace ProcfilerLoggerProvider;

internal class ProcfilerLogger(IOptionsMonitor<ProcfilerLoggerConfiguration> configuration) : ILogger
{
  public void Log<TState>(LogLevel level, EventId eventId, TState state, Exception? exception, Func<TState, Exception?, string> formatter)
  {
    ProcfilerBusinessEventsSource.Instance.WriteBusinessEvent(level, eventId);
  }

  public bool IsEnabled(LogLevel level)
  {
    if (level == LogLevel.None) return false;
    
    return level < configuration.CurrentValue.LogLevel;
  }

  public IDisposable? BeginScope<TState>(TState state) where TState : notnull => default;
}