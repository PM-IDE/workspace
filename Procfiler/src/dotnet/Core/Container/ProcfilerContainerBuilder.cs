using System.Reflection;
using Autofac;
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
      .AsImplementedInterfaces()
      .SingleInstance();

    var logger = LoggerFactory.Create(options =>
    {
      options.SetMinimumLevel(logLevel);
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