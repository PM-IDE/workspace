using Core.CommandLine;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsCollection.ModificationSources;

namespace Procfiler.Core.EventsCollection;

public interface IFreezableCollection
{
  void Freeze();
  void UnFreeze();
}

public class CollectionIsFrozenException : ProcfilerException;

public interface IInsertableEventsCollection
{
  EventPointer InsertAfter(EventPointer pointer, EventRecordWithMetadata eventToInsert);
  EventPointer InsertBefore(EventPointer pointer, EventRecordWithMetadata eventToInsert);
}

public interface IRemovableEventsCollection
{
  bool Remove(EventPointer pointer);
}

public interface IMutableEventsCollection : IRemovableEventsCollection, IInsertableEventsCollection
{
  void AddFilter(Predicate<EventRecordWithMetadata> filter);
}

public interface IEventsOwner : IMutableEventsCollection, IFreezableCollection, IEnumerable<EventRecordWithPointer>
{
  long Count { get; }
}

public interface IEventsCollection : IEventsOwner
{
  void ApplyNotPureActionForAllEvents(Func<EventRecordWithPointer, bool> action);
  void InjectModificationSource(IModificationSource modificationSource);
}