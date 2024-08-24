using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Methods;

public abstract class MethodInliningNameMutatorBase(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForTypeLikeName(
      TraceEventsConstants.MethodInliningSucceededInlineeNamespace, EventClassKind.Zero),
    MetadataKeysWithTransform.CreateForTypeLikeName(
      TraceEventsConstants.MethodInliningSucceededInlineeName, EventClassKind.Zero)
  };
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class MethodInliningFailedNameMutator(IProcfilerLogger logger) : MethodInliningNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.MethodInliningFailed;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class MethodInliningSucceededNameMutator(IProcfilerLogger logger) : MethodInliningNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.MethodInliningSucceeded;
}