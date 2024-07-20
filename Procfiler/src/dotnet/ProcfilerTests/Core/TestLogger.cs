using Core.Utils;
using Microsoft.Extensions.Logging;
using Procfiler.Utils;

namespace ProcfilerTests.Core;

public class TestLogger : IProcfilerLogger
{
  public static TestLogger CreateInstance() => new();


  public void Log<TState>(
    LogLevel logLevel, EventId eventId, TState state, Exception? exception, Func<TState, Exception?, string> formatter)
  {
    if (logLevel == LogLevel.Error) Assert.Fail($"Logging error, {state}, {exception}");
  }

  public bool IsEnabled(LogLevel logLevel) => true;
  public IDisposable? BeginScope<TState>(TState state) where TState : notnull => null;

  public void IncreaseIndent()
  {
  }

  public void DecreaseIndent()
  {
  }
}