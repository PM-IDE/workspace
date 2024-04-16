using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsCollection;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.SplitByMethod;

public interface IEventsCollectionByMethodsSplitter
{
  IReadOnlyDictionary<string, IReadOnlyList<IReadOnlyList<EventRecordWithMetadata>>> Split(
    IEventsCollection events,
    string filterPattern,
    InlineMode inlineEventsFromInnerMethods);
}

[AppComponent]
public class EventsCollectionByMethodsSplitterImpl(IProcfilerLogger logger, IProcfilerEventsFactory eventsFactory) : IEventsCollectionByMethodsSplitter
{
  public IReadOnlyDictionary<string, IReadOnlyList<IReadOnlyList<EventRecordWithMetadata>>> Split(
    IEventsCollection events,
    string filterPattern,
    InlineMode inlineEventsFromInnerMethods)
  {
    return new SplitterImplementation(logger, eventsFactory, events, filterPattern, inlineEventsFromInnerMethods).Split();
  }
}