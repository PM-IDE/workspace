using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Methods;

public abstract class MethodLoadUnloadNameMutatorBase(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForTypeLikeName(TraceEventsConstants.MethodNamespace, EventClassKind.Zero),
    MetadataKeysWithTransform.CreateForTypeLikeName(TraceEventsConstants.MethodName, EventClassKind.Zero)
  };
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class MethodLoadVerboseNameMutator(IProcfilerLogger logger) : MethodLoadUnloadNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.MethodLoadVerbose;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class MethodUnloadVerboseNameMutator(IProcfilerLogger logger) : MethodLoadUnloadNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.MethodUnloadVerbose;
}