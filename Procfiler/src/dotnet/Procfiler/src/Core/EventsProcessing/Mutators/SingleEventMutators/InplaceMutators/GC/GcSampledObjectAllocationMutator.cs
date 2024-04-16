using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;
using Procfiler.Utils.Container;

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