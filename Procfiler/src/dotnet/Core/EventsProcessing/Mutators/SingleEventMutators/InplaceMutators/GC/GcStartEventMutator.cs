using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class GcStartEventMutator : MetadataValueToNameAppenderBase
{
  public override string EventType => TraceEventsConstants.GcStart;
  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; }


  public GcStartEventMutator(IProcfilerLogger logger) : base(logger)
  {
    Transformations =
    [
      new MetadataKeysWithTransform(TraceEventsConstants.GcStartReason, reason => GcMutatorsUtil.TransformGcReason(reason, logger), EventClassKind.Zero),
      new MetadataKeysWithTransform(TraceEventsConstants.GcStartType, type => GcMutatorsUtil.TransformGcType(type, Logger), EventClassKind.Zero)
    ];
  }
}