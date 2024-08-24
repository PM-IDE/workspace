using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection;

namespace Procfiler.Core.SplitByMethod;

public interface IEventsCollectionByMethodsSplitter
{
  IReadOnlyDictionary<string, List<List<EventRecordWithMetadata>>> Split(
    IEventsCollection events,
    string filterPattern,
    InlineMode inlineEventsFromInnerMethods);
}

[AppComponent]
public class EventsCollectionByMethodsSplitterImpl(IProcfilerLogger logger, IProcfilerEventsFactory eventsFactory)
  : IEventsCollectionByMethodsSplitter
{
  public IReadOnlyDictionary<string, List<List<EventRecordWithMetadata>>> Split(
    IEventsCollection events,
    string filterPattern,
    InlineMode inlineEventsFromInnerMethods) =>
    new SplitterImplementation(logger, eventsFactory, events, filterPattern, inlineEventsFromInnerMethods).Split();
}