using Ficus;

namespace FicusFrontend.Components.CaseInfo;

internal static class Utils
{
  public static T? TryGetContextValue<T>(
    this ILogger logger,
    string pipelinePartName,
    List<GrpcContextValue> contextValues,
    GrpcContextValue.ContextValueOneofCase cvCase,
    Func<GrpcContextValue, T> selector) where T : class
  {
    var value = contextValues.FirstOrDefault(c => c.ContextValueCase == cvCase);

    if (value is { }) return selector(value);

    logger.LogError("{Case} was null for pipeline part {PipelinePartName}", cvCase, pipelinePartName);
    return null;
  }
}