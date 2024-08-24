using Autofac;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Configuration.EnvironmentVariables;
using Microsoft.Extensions.Options;
using ProcfilerOnline.Core.Settings;

namespace ProcfilerOnline.Core.Container;

public static class ConfigurationUtil
{
  public static void AddConfiguration(ContainerBuilder containerBuilder)
  {
    var configurationBuilder = new ConfigurationBuilder();
    configurationBuilder.Add(new EnvironmentVariablesConfigurationSource());

    var configuration = configurationBuilder.Build();
    containerBuilder.RegisterInstance(configuration).As<IConfiguration>();

    var procfilerSettings = configuration.GetSection(nameof(OnlineProcfilerSettings)).Get<OnlineProcfilerSettings>() ??
                            new OnlineProcfilerSettings();

    containerBuilder.RegisterInstance(Options.Create(procfilerSettings ?? throw new Exception()));
  }
}