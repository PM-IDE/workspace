using Autofac;
using Core.Container;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Configuration.EnvironmentVariables;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using ProcfilerOnline.Core.Settings;

Environment.SetEnvironmentVariable("OnlineProcfilerSettings__KafkaSettings__TopicName", "my-topic");
Environment.SetEnvironmentVariable("OnlineProcfilerSettings__KafkaSettings__BootstrapServers", "localhost:9092");

ProgramEntryPoint.SetupContainerAndRun("procfiler-online", args, AddConfiguration, LogLevel.Debug);

void AddConfiguration(ContainerBuilder containerBuilder)
{
  var configurationBuilder = new ConfigurationBuilder();
  configurationBuilder.Add(new EnvironmentVariablesConfigurationSource());

  var configuration = configurationBuilder.Build();
  containerBuilder.RegisterInstance(configuration).As<IConfiguration>();

  var procfilerSettings = configuration.GetSection(nameof(OnlineProcfilerSettings)).Get<OnlineProcfilerSettings>();
  containerBuilder.RegisterInstance(Options.Create(procfilerSettings ?? throw new Exception()));
}