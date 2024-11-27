// See https://aka.ms/new-console-template for more information

using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using ProcfilerLoggerProvider;

foreach (var configLogLevel in Enum.GetValues<LogLevel>())
{
  var configuration = new ProcfilerLoggerConfiguration
  {
    LogLevel = configLogLevel
  };

  var provider = new ProcfilerLoggerProvider.ProcfilerLoggerProvider(new MyOptionsMonitor(configuration));
  var logger = provider.CreateLogger(string.Empty);

  foreach (var currentLogLevel in Enum.GetValues<LogLevel>())
  {
    logger.Log(currentLogLevel, "Config log level: {ConfigLogLevel}, current log level: {CurrentLogLevel}", configLogLevel, currentLogLevel);
  }
}

class MyOptionsMonitor(ProcfilerLoggerConfiguration configuration) : IOptionsMonitor<ProcfilerLoggerConfiguration>
{
  public ProcfilerLoggerConfiguration CurrentValue { get; } = configuration;


  public ProcfilerLoggerConfiguration Get(string? name)
  {
    return configuration;
  }

  public IDisposable? OnChange(Action<ProcfilerLoggerConfiguration, string?> listener)
  {
    return default;
  }
}