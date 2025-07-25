using System.Text;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using ProcfilerEventSources;

namespace ProcfilerLoggerProvider;

internal class ProcfilerLogger(IOptionsMonitor<ProcfilerLoggerConfiguration> configuration) : ILogger
{
  public void Log<TState>(
    LogLevel level, EventId eventId, TState state, Exception? exception, Func<TState, Exception?, string> formatter)
  {
    if (!IsEnabled(level)) return;

    var (attributes, message) = state switch
    {
      IEnumerable<KeyValuePair<string, object?>> e => CreateAttributeStringAndMessage(e, state, exception, formatter),
      _ => (string.Empty, formatter(state, exception))
    };

    ProcfilerBusinessEventsSource.Instance.BusinessEvent((int)level, message, attributes);
  }


  private (string, string?) CreateAttributeStringAndMessage<TState>(
    IEnumerable<KeyValuePair<string, object?>> attributes, TState state, Exception? exception, Func<TState, Exception?, string> formatter)
  {
    var sb = new StringBuilder();
    string? message = null;
    var foundOriginalFormat = false;

    foreach (var (key, value) in attributes)
    {
      sb.Append(key)
        .Append(';')
        .Append(value)
        .Append(';');

      if (configuration.CurrentValue.MessageLogKind is MessageLogKind.OriginalFormat && key is "{OriginalFormat}")
      {
        message = value?.ToString();
        foundOriginalFormat = true;
      }
    }

    if (sb.Length > 0)
    {
      sb.Remove(sb.Length - 1, 1);
    }

    message ??= (configuration.CurrentValue.MessageLogKind, foundOriginalFormat) switch
    {
      (MessageLogKind.Message, _) => formatter(state, exception),
      (MessageLogKind.OriginalFormat, false) => "Unspecified original format",
      _ => throw new ArgumentOutOfRangeException()
    };

    return (sb.ToString(), message);
  }

  public bool IsEnabled(LogLevel level)
  {
    if (level == LogLevel.None) return false;

    return level >= configuration.CurrentValue.LogLevel;
  }

  public IDisposable? BeginScope<TState>(TState state) where TState : notnull => default;
}