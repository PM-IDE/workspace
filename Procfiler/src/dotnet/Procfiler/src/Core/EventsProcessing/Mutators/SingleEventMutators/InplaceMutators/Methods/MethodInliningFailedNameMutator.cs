using Core.Container;
using Core.Utils;
using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Methods;

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