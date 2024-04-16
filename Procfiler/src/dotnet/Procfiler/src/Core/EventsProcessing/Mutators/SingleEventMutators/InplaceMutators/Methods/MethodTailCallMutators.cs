using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Methods;

public abstract class MethodTailCallNameMutatorBase(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForTypeLikeName(TraceEventsConstants.MethodBeingCompiledNamespace, EventClassKind.Zero),
    MetadataKeysWithTransform.CreateForTypeLikeName(TraceEventsConstants.MethodBeingCompiledName, EventClassKind.Zero)
  };
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class MethodTailCallSucceededNameMutator(IProcfilerLogger logger) : MethodTailCallNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.MethodTailCallSucceeded;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class MethodTailCallFailedNameMutator(IProcfilerLogger logger) : MethodTailCallNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.MethodTailCallFailed;
}