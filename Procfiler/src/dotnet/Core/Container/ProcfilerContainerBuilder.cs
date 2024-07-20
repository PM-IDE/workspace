using System.Reflection;
using Autofac;
using Core.Utils;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Console;

namespace Core.Container;

public static class ProcfilerContainerBuilder
{
  public static ContainerBuilder BuildFromAssembly(LogLevel logLevel, IReadOnlyList<Assembly> assemblies)
  {
    var builder = new ContainerBuilder();
    builder.RegisterAssemblyTypes(assemblies.ToArray())
      .Where(t => t.IsClass && t.GetCustomAttribute<AppComponentAttribute>() is { })
      .AsImplementedInterfaces();

    var logger = LoggerFactory.Create(options =>
    {
      options.SetMinimumLevel(logLevel);
      var logDirectory = Path.Combine(Directory.GetCurrentDirectory(), "Logs");
      PathUtils.EnsureEmptyDirectoryOrThrow(logDirectory);
      var logFile = Path.Combine(logDirectory, $"log-{{Date}}-{DateTime.Now.TimeOfDay.TotalMilliseconds}.txt");
      options.AddFile(logFile, logLevel);
      options.AddSimpleConsole(formatterOptions =>
      {
        formatterOptions.SingleLine = true;
        formatterOptions.IncludeScopes = false;
        formatterOptions.ColorBehavior = LoggerColorBehavior.Enabled;
      });
    }).CreateLogger(string.Empty);

    builder.RegisterInstance(logger);
    return builder;
  }
}