using Core.Constants.TraceEvents;
using Core.Container;
using Core.Events.EventRecord;
using Core.EventsProcessing.Mutators;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.GlobalData;
using Core.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Gc;

[EventMutator(SingleEventMutatorsPasses.GcStopTypeSetter)]
public class GcEndTypeMutator(IProcfilerLogger logger) : ISingleEventMutator
{
  private readonly Dictionary<string, string> myCountToTypes = [];


  public IEnumerable<EventLogMutation> Mutations { get; } =
  [
    new AttributeToNameAppendMutation(TraceEventsConstants.GcStop, EventClassKind.Zero, TraceEventsConstants.GcStartType, false),
    new AttributeToNameAppendMutation(TraceEventsConstants.GcStop, EventClassKind.Zero, TraceEventsConstants.GcStartReason, false),
  ];


  public void Process(EventRecordWithMetadata eventRecord, IGlobalData context)
  {
    switch (eventRecord.EventClass)
    {
      case TraceEventsConstants.GcStart:
      {
        var count = eventRecord.Metadata[TraceEventsConstants.GcCount];

        if (myCountToTypes.ContainsKey(count))
        {
          logger.LogWarning("GC with count {Count} already exists in the map", count);
        }

        var gcType = eventRecord.Metadata[TraceEventsConstants.GcStartType];
        var gcReason = eventRecord.Metadata[TraceEventsConstants.GcStartReason];

        myCountToTypes[count] = $"_{{{MutatorsUtil.TransformGcType(gcType, logger)}{gcReason}}}";
        return;
      }
      case TraceEventsConstants.GcStop:
      {
        var count = eventRecord.Metadata[TraceEventsConstants.GcCount];
        if (!myCountToTypes.TryGetValue(count, out var type))
        {
          logger.LogWarning("There is no GC type for GC number {Count}", count);
          return;
        }

        eventRecord.EventName += type;
        return;
      }
    }
  }
}