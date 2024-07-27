using Core.Constants.TraceEvents;
using Core.Container;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord.EventRecord;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

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
    Mutations = new[]
    {
      new NewAttributeCreationMutation(EventClass, TraceEventsConstants.GcSampledObjectAllocationTypeName)
    };
  }


  public void Process(EventRecordWithMetadata eventRecord, SessionGlobalData context)
  {
    if (eventRecord.EventClass is TraceEventsConstants.GcSampledObjectAllocation &&
        eventRecord.Metadata.GetValueOrDefault(TraceEventsConstants.GcSampledObjectAllocTypeId) is { } id)
    {
      if (context.TypeIdToNames.TryGetValue(id.ParseId(), out var typeName))
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
}