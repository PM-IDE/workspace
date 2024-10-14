using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Methods;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class MethodJittingStartedNameMutator(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  public override string EventType => TraceEventsConstants.MethodJittingStarted;

  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; } =
  [
    MetadataKeysWithTransform.CreateForTypeLikeName(TraceEventsConstants.MethodNamespace, EventClassKind.Zero),
    MetadataKeysWithTransform.CreateForTypeLikeName(TraceEventsConstants.MethodName, EventClassKind.Zero)
  ];
}