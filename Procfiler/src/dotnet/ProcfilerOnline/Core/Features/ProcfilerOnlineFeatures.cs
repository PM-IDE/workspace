using Core.Features;

namespace ProcfilerOnline.Core.Features;

public static class ProcfilerOnlineFeatures
{
  public static Feature ProduceEventsToKafka { get; } =
    new EnvironmentVariableFeature(nameof(ProduceEventsToKafka), nameof(ProduceEventsToKafka));
}