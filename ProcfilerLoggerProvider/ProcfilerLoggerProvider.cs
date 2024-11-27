using System.Collections.Concurrent;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.DependencyInjection.Extensions;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Configuration;
using Microsoft.Extensions.Options;

namespace ProcfilerLoggerProvider;

public class ProcfilerLoggerConfiguration
{
  public LogLevel LogLevel { get; set; }
}

public static class ServiceCollectionsExtensions
{
  public static ILoggingBuilder AddProcfilerLogger(this ILoggingBuilder builder, Action<ProcfilerLoggerConfiguration> configFunc)
  {
    builder.Services.TryAddEnumerable(ServiceDescriptor.Singleton<ILoggerProvider, ProcfilerLoggerProvider>());
    LoggerProviderOptions.RegisterProviderOptions<ProcfilerLoggerConfiguration, ProcfilerLoggerProvider>(builder.Services);
    builder.Services.Configure(configFunc);

    return builder;
  }
}

public class ProcfilerLoggerProvider(IOptionsMonitor<ProcfilerLoggerConfiguration> config) : ILoggerProvider
{
  private readonly ConcurrentDictionary<string, ProcfilerLogger> myLoggers = [];


  public ILogger CreateLogger(string categoryName) => myLoggers.GetOrAdd(categoryName, _ => new ProcfilerLogger(config));


  public void Dispose()
  {
    myLoggers.Clear();
  }
}