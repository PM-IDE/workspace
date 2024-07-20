using Core.Container;
using Core.Utils;
using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class GcTriggeredEventMutator : MetadataValueToNameAppenderBase
{
  public override string EventType => TraceEventsConstants.GcTriggered;
  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; }


  public GcTriggeredEventMutator(IProcfilerLogger logger) : base(logger)
  {
    string TransformReason(string reason) => GcMutatorsUtil.GenerateNewNameForGcReason(reason, Logger);

    Transformations = new[]
    {
      new MetadataKeysWithTransform(TraceEventsConstants.CommonReason, TransformReason, EventClassKind.Zero)
    };
  }
}