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
    const string KeyAttribute = TraceEventsConstants.BusinessEventOriginalFormat;
    const string AttributesSeparator = ";";

    if (eventRecord.Metadata.TryGetValue(TraceEventsConstants.BusinessEventAttributes, out var attributesString))
    {
      var attributes = attributesString.Split(AttributesSeparator);
      if (attributes.Length % 2 == 0)
      {
        foreach (var (key, value) in attributes.Where((_, i) => i % 2 == 0).Zip(attributes.Where((_, i) => i % 2 == 1)))
        {
          eventRecord.Metadata[key] = value;
        }
      }
      else
      {
        logger.LogWarning("Attributes key values count is not % 2 == 0");
      }
    }

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