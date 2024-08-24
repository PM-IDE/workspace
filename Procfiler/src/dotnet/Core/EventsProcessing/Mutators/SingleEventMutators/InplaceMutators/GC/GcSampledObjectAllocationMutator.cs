using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

[EventMutator(MultipleEventMutatorsPasses.LastMultipleMutators)]
public class GcSampledObjectAllocationMutator(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForTypeLikeName(TraceEventsConstants.GcSampledObjectAllocationTypeName, EventClassKind.Zero)
  };


  public override string EventType => TraceEventsConstants.GcSampledObjectAllocation;
}