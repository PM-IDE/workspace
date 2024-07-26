using Core.Constants.TraceEvents;
using Core.Container;
using Core.Utils;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class GcFinalizeObjectMutator(IProcfilerLogger logger) : MetadataValuesRemover(logger)
{
  protected override string[] MetadataKeys { get; } =
  [
    TraceEventsConstants.CommonTypeId,
    TraceEventsConstants.CommonObjectId
  ];

  public override string EventType => TraceEventsConstants.GcFinalizeObject;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class GcFinalizeObjectNameMutator(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  public override string EventType => TraceEventsConstants.GcFinalizeObject;

  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForTypeLikeName(TraceEventsConstants.CommonTypeName, EventClassKind.Zero)
  };
}