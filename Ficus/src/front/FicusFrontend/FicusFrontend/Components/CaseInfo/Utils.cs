using Ficus;
using FicusFrontend.Services.Cases;

namespace FicusFrontend.Components.CaseInfo;

internal static class Utils
{
  public static (Guid, T)? TryGetContextValue<T>(
    this ILogger logger,
    string pipelinePartName,
    List<ContextValueWrapper> contextValues,
    GrpcContextValue.ContextValueOneofCase cvCase,
    Func<GrpcContextValue, T> selector) where T : class
  {
    var value = contextValues.FirstOrDefault(c => c.Value.Value.ContextValueCase == cvCase);

    if (value is { })
    {
      return (value.Id, selector(value.Value.Value));
    }

    logger.LogError("{Case} was null for pipeline part {PipelinePartName}", cvCase, pipelinePartName);
    return null;
  }
}