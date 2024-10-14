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
    string TransformReason(string reason) => GcMutatorsUtil.GenerateNewNameForGcReason(reason, Logger);

    Transformations =
    [
      new MetadataKeysWithTransform(TraceEventsConstants.GcStartReason, TransformReason, EventClassKind.Zero),
      new MetadataKeysWithTransform(TraceEventsConstants.GcStartType, GenerateNameForGcType, EventClassKind.Zero)
    ];
  }


  private string GenerateNameForGcType(string type) => type switch
  {
    "NonConcurrentGC" => "NC_GC",
    "BackgroundGC" => "B_GC",
    "ForegroundGC" => "F_GC",
    _ => MutatorsUtil.CreateUnknownEventNamePartAndLog(type, Logger)
  };
}