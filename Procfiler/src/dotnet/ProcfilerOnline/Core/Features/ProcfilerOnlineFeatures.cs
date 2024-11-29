using Core.Features;

namespace ProcfilerOnline.Core.Features;

public static class ProcfilerOnlineFeatures
{
  public static Feature ProduceEventsToKafka { get; } =
    new EnvironmentVariableFeature(nameof(ProduceEventsToKafka), nameof(ProduceEventsToKafka));

  public static Feature ProduceBxesKafkaEvents { get; } =
    new EnvironmentVariableFeature(nameof(ProduceBxesKafkaEvents), nameof(ProduceBxesKafkaEvents));
  
  public static Feature ProduceGcEvents { get; } = new EnvironmentVariableFeature(nameof(ProduceGcEvents), nameof(ProduceGcEvents), defaultValue: false);
}