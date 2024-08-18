namespace ProcfilerOnline.Core.Settings;

public class OnlineProcfilerSettings
{
  public KafkaSettings KafkaSettings { get; init; } = new();
}

public class KafkaSettings
{
  public string TopicName { get; init; }
  public string BootstrapServers { get; init; }
}