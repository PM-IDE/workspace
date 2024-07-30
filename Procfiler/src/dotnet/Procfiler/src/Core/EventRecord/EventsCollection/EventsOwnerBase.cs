using Core.Events.EventRecord;
using Core.Utils;

namespace Procfiler.Core.EventRecord.EventsCollection;

public abstract class EventsOwnerBase : IEventsOwner
{
  private readonly IProcfilerLogger myLogger;
  private readonly List<Predicate<EventRecordWithMetadata>> myFilters = [];

  protected readonly EventPointersManager PointersManager;

  private bool myIsFrozen;


  public abstract long Count { get; }


  protected EventsOwnerBase(IProcfilerLogger logger, long initialEventsCount)
  {
    myLogger = logger;
    PointersManager = new EventPointersManager(initialEventsCount, new InsertedEvents(), this);
  }


  public virtual IEnumerator<EventRecordWithPointer> GetEnumerator() => EnumerateInternal().GetEnumerator();

  protected IEnumerable<EventRecordWithPointer> EnumerateInternal()
  {
    using var initialEventsEnumerator = EnumerateInitialEvents().GetEnumerator();
    if (!initialEventsEnumerator.MoveNext())
    {
      myLogger.LogWarning("The enumerator is empty for {Type}", GetType().Name);
      yield break;
    }

    var currentIndex = 0;

    var current = PointersManager.First;
    while (current is { })
    {
      if (PointersManager.TryGetInsertedEvent(current.Value) is { } insertedEvent)
      {
        if (!PointersManager.IsRemoved(current.Value) && !ShouldFilter(insertedEvent))
        {
          yield return new EventRecordWithPointer
          {
            Event = insertedEvent,
            EventPointer = current.Value
          };
        }
      }
      else
      {
        Debug.Assert(current.Value.IsInInitialArray);
        while (currentIndex != current.Value.IndexInArray)
        {
          if (!initialEventsEnumerator.MoveNext())
          {
            myLogger.LogWarning("The enumerator finished before pointer, maybe corrupted stacks in {Type}", GetType().Name);
            yield break;
          }

          ++currentIndex;
        }

        if (!PointersManager.IsRemoved(current.Value) && !ShouldFilter(initialEventsEnumerator.Current))
        {
          yield return new EventRecordWithPointer
          {
            Event = initialEventsEnumerator.Current,
            EventPointer = current.Value
          };
        }
      }

      current = PointersManager.Next(current.Value);
    }
  }

  public virtual EventPointer InsertAfter(EventPointer pointer, EventRecordWithMetadata eventToInsert) =>
    PointersManager.InsertAfter(pointer, eventToInsert);

  public virtual EventPointer InsertBefore(EventPointer pointer, EventRecordWithMetadata eventToInsert) =>
    PointersManager.InsertBefore(pointer, eventToInsert);

  public void AddFilter(Predicate<EventRecordWithMetadata> filter)
  {
    myFilters.Add(filter);
  }

  private bool ShouldFilter(EventRecordWithMetadata eventRecord)
  {
    if (myFilters.Count == 0) return false;

    foreach (var predicate in myFilters)
    {
      if (predicate(eventRecord)) return true;
    }

    return false;
  }

  public abstract bool Remove(EventPointer pointer);

  public void Freeze() => myIsFrozen = true;
  public void UnFreeze() => myIsFrozen = false;

  protected void AssertNotFrozen()
  {
    if (myIsFrozen) throw new CollectionIsFrozenException();
  }

  IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

  protected abstract IEnumerable<EventRecordWithMetadata> EnumerateInitialEvents();
}