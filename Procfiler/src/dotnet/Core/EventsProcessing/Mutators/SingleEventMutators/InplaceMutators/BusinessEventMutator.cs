using Core.Constants.TraceEvents;
using Core.Container;
using Core.Events.EventRecord;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.GlobalData;
using Core.Utils;
using Microsoft.Extensions.Logging;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class BusinessEventMutator(IProcfilerLogger logger) : SingleEventMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.BusinessEvent;
  public override IEnumerable<EventLogMutation> Mutations { get; } = [];


  protected override void ProcessInternal(EventRecordWithMetadata eventRecord, IGlobalData context)
  {
    const string KeyAttribute = TraceEventsConstants.BusinessEventMessage;

    if (eventRecord.Metadata.TryGetValue(KeyAttribute, out var message))
    {
      eventRecord.EventName = $"{TraceEventsConstants.BusinessEvent}[{message}]";
    }
    else
    {
      Logger.LogWarning("The {Attribute} was not present in business event, will not change its name", KeyAttribute);
    }
  }
}