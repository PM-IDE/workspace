using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class GcCreateSegmentMutator(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; } =
  [
    new(TraceEventsConstants.GcSegmentType, type => type, EventClassKind.Zero)
  ];


  public override string EventType => TraceEventsConstants.GcCreateSegment;
}