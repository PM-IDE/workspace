using Confluent.Kafka;
using Microsoft.Extensions.Logging;

namespace FicusKafkaIntegration;

public static class KafkaUtils
{
  public static void WaitUntilTopicExists(this ILogger logger, string bootstrapServers, string topicName)
  {
    var config = new AdminClientConfig
    {
      BootstrapServers = bootstrapServers
    };

    using var client = new AdminClientBuilder(config).Build();

    try
    {
      while (!client.GetMetadata(TimeSpan.FromSeconds(5)).Topics.Select(t => t.Topic).ToHashSet().Contains(topicName))
      {
        logger.LogInformation("The topic is not created, will wait");
        Thread.Sleep(TimeSpan.FromSeconds(1));
      }
    }
    catch (Exception ex)
    {
      logger.LogError(ex, "Failed to get metadata");
    }
  }
}