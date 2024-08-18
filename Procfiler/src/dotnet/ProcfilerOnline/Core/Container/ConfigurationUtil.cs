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
    const string ProduceEventsToKafkaSetting = "OnlineProcfilerSettings__ProduceEventsToKafka";
    if (Environment.GetEnvironmentVariable(ProduceEventsToKafkaSetting) is not { })
    {
      Environment.SetEnvironmentVariable(ProduceEventsToKafkaSetting, false.ToString());
    }

    var configurationBuilder = new ConfigurationBuilder();
    configurationBuilder.Add(new EnvironmentVariablesConfigurationSource());

    var configuration = configurationBuilder.Build();
    containerBuilder.RegisterInstance(configuration).As<IConfiguration>();

    var procfilerSettings = configuration.GetSection(nameof(OnlineProcfilerSettings)).Get<OnlineProcfilerSettings>();
    containerBuilder.RegisterInstance(Options.Create(procfilerSettings ?? throw new Exception()));
  }
}