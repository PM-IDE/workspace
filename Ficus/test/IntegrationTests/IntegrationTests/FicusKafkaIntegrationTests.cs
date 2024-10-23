using Bxes.Kafka;
using Bxes.Models.Domain;
using Bxes.Models.System;
using Bxes.Utils;
using Confluent.Kafka;

namespace IntegrationTests;

internal static class EnvVars
{
  public const string BootstrapServers = nameof(BootstrapServers);
  public const string TopicName = nameof(TopicName);

  public static string GetEnvOrThrow(string envVarName) => Environment.GetEnvironmentVariable(envVarName) ??
                                                           throw new Exception($"Env var {envVarName} is not set");
} 

public class FicusKafkaIntegrationTests
{
  [Test]
  public void DoTest()
  {
    var eventLog = RandomLogsGenerator.CreateSimpleLog();
    var writer = CreateBxesKafkaWriter();
    foreach (var @event in eventLog.ToEventsStream())
    {
      writer.HandleEvent(@event);
    }
    
    
  }

  private BxesKafkaStreamWriter<IEvent> CreateBxesKafkaWriter() => new(
    new SystemMetadata(),
    EnvVars.GetEnvOrThrow(EnvVars.TopicName),
    new ProducerConfig
    {
      BootstrapServers = EnvVars.GetEnvOrThrow(EnvVars.BootstrapServers)
    }
  );
}