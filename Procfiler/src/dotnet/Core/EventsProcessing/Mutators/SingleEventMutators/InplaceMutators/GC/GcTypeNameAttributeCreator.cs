using Core.Constants.TraceEvents;
using Core.Container;
using Core.Events.EventRecord;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.GlobalData;
using Core.Utils;
using Microsoft.Extensions.Logging;
using ProcfilerLoggerProvider;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

[EventMutator(SingleEventMutatorsPasses.AttributesCreators)]
public class GcTypeNameAttributeCreator : ISingleEventMutator
{
  private readonly IProcfilerLogger myLogger;


  public IEnumerable<EventLogMutation> Mutations { get; }
  public string EventClass { get; }


  public GcTypeNameAttributeCreator(IProcfilerLogger logger)
  {
    myLogger = logger;
    EventClass = TraceEventsConstants.GcSampledObjectAllocation;
    Mutations =
    [
      new NewAttributeCreationMutation(EventClass, TraceEventsConstants.GcSampledObjectAllocationTypeName)
    ];
  }


  public void Process(EventRecordWithMetadata eventRecord, IGlobalData context)
  {
    if (eventRecord.EventClass is not TraceEventsConstants.GcSampledObjectAllocation ||
        eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.GcSampledObjectAllocTypeId) is not { } id)
    {
      return;
    }

    OcelLogger.LogGloballyAttachedObject(eventRecord, GetType().Name, eventRecord.EventClass);

    if (context.FindTypeName(id.ParseId()) is { } typeName)
    {
      eventRecord.Metadata[TraceEventsConstants.GcSampledObjectAllocationTypeName] = typeName;
    }
    else
    {
      myLogger.LogTrace("Failed to find type name for type id {Id}", id);
      eventRecord.Metadata[TraceEventsConstants.GcSampledObjectAllocationTypeName] = "UNRESOLVED";
    }
  }
}