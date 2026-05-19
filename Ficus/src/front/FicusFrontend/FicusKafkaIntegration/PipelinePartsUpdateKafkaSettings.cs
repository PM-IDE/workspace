namespace FicusKafkaIntegration;

public class PipelinePartsUpdateKafkaSettings
{
  public required string Topic { get; init; }
  public required string BootstrapServers { get; init; }
}