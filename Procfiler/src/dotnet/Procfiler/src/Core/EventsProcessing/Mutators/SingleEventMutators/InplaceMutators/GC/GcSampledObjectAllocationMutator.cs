using Core.Constants.TraceEvents;
using Core.Container;
using Core.Utils;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

[EventMutator(MultipleEventMutatorsPasses.LastMultipleMutators)]
public class GcSampledObjectAllocationMutator(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForTypeLikeName(TraceEventsConstants.GcSampledObjectAllocationTypeName, EventClassKind.Zero)
  };


  public override string EventType => TraceEventsConstants.GcSampledObjectAllocation;
}