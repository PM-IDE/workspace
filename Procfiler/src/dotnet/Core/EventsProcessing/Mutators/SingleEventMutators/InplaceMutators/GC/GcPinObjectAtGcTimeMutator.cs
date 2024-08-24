using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class GcPinObjectAtGcTimeMutator(IProcfilerLogger logger) : MetadataValuesRemover(logger)
{
  public override string EventType => TraceEventsConstants.GcPinObjectAtGcTime;

  protected override string[] MetadataKeys { get; } =
  [
    TraceEventsConstants.CommonObjectId,
    TraceEventsConstants.CommonHandleId
  ];
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class GcPinObjectAtGcTimeNameMutator(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  public override string EventType => TraceEventsConstants.GcPinObjectAtGcTime;

  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForTypeLikeName(TraceEventsConstants.CommonTypeName, EventClassKind.Zero)
  };
}