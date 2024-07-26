using Core.Events.EventsCollection;

namespace Procfiler.Core.EventsProcessing.Filters.Core;

public interface IEventsFilter
{
  IEnumerable<string> AllowedEventsNames { get; }

  void Filter(IEventsCollection events);
}