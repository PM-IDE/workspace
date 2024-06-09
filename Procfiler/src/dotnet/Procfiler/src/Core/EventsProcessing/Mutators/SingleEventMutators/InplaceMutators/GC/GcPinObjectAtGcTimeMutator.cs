using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

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