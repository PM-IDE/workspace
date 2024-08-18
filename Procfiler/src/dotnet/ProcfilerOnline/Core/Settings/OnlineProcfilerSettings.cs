namespace ProcfilerOnline.Core.Settings;

public class OnlineProcfilerSettings
{
  public required KafkaSettings KafkaSettings { get; init; }
}

public class KafkaSettings
{
  public required string TopicName { get; init; }
  public required string BootstrapServers { get; init; }
}