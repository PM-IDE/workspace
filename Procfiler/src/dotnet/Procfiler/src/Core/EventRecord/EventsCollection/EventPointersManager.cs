using Core.Events.EventRecord;

namespace Procfiler.Core.EventRecord.EventsCollection;

public class EventPointersManager(long initialCount, InsertedEvents insertedEvents, IEventsOwner owner)
{
  private readonly HashSet<int> myRemovedIndexesInInitialArray = [];

  private long myCurrentInitialEventsCount = initialCount;

  public long Count => myCurrentInitialEventsCount + insertedEvents.Count;


  public EventPointer? First
  {
    get
    {
      EventPointer? startPointer = insertedEvents.FirstEvents switch
      {
        { } => EventPointer.ForFirstEvent(0, owner),
        _ => EventPointer.ForInitialArray(0, owner)
      };

      return startPointer;
    }
  }

  public EventPointer InsertAfter(EventPointer pointer, EventRecordWithMetadata eventToInsert)
  {
    AssertStateOrThrow(pointer);
    insertedEvents.InsertAfter(pointer, eventToInsert);

    var insertedEventPointer = Next(pointer);
    Debug.Assert(insertedEventPointer is { });
    return insertedEventPointer.Value;
  }

  public EventPointer InsertBefore(EventPointer pointer, EventRecordWithMetadata eventToInsert)
  {
    AssertStateOrThrow(pointer);
    insertedEvents.InsertBefore(pointer, eventToInsert);

    var insertedEventPointer = PrevInternal(pointer);
    Debug.Assert(insertedEventPointer is { });
    return insertedEventPointer.Value;
  }

  private void DecreaseCount(long delta = 1) => myCurrentInitialEventsCount -= delta;

  public bool Remove(EventPointer pointer)
  {
    if (pointer.IsInInitialArray)
    {
      if (myRemovedIndexesInInitialArray.Contains(pointer.IndexInArray)) return false;

      myRemovedIndexesInInitialArray.Add(pointer.IndexInArray);
      DecreaseCount();
      return true;
    }

    return insertedEvents.Remove(pointer);
  }

  public bool IsRemoved(EventPointer pointer) => pointer.IsInInitialArray switch
  {
    true => myRemovedIndexesInInitialArray.Contains(pointer.IndexInArray),
    false => insertedEvents.IsRemoved(pointer)
  };

  public EventRecordWithMetadata? TryGetInsertedEvent(EventPointer pointer)
  {
    AssertStateOrThrow(pointer);
    return pointer.IsInInsertedMap switch
    {
      true => pointer.IsInFirstEvents switch
      {
        true => insertedEvents.FirstEvents![pointer.IndexInInsertionMap],
        false => insertedEvents.GetOrThrow(pointer)
      },
      _ => null
    };
  }

  public EventPointer? Next(in EventPointer current)
  {
    AssertStateOrThrow(current);
    return NextInternal(current);
  }

  private EventPointer? NextInternal(in EventPointer current)
  {
    if (current.IsInInsertedMap)
    {
      if (current.IsInFirstEvents)
      {
        Debug.Assert(insertedEvents.FirstEvents is { });
        if (current.IndexInInsertionMap == insertedEvents.FirstEvents.Count - 1)
        {
          if (initialCount == 0) return null;

          return EventPointer.ForInitialArray(0, owner);
        }

        return EventPointer.ForFirstEvent(current.IndexInInsertionMap + 1, owner);
      }

      var insertedEventsList = GetInsertedEventsListOrThrow(current);
      if (current.IndexInInsertionMap == insertedEventsList.Count - 1)
      {
        var nextInitialArrayIndex = current.IndexInArray + 1;
        if (nextInitialArrayIndex >= initialCount) return null;

        return EventPointer.ForInitialArray(nextInitialArrayIndex, owner);
      }

      return EventPointer.ForInsertionMap(current.IndexInArray, current.IndexInInsertionMap + 1, owner);
    }

    if (insertedEvents.ContainsKey(current.IndexInArray))
    {
      return EventPointer.ForInsertionMap(current.IndexInArray, 0, owner);
    }

    var nextArrayIndex = current.IndexInArray + 1;
    if (nextArrayIndex >= initialCount) return null;

    return EventPointer.ForInitialArray(nextArrayIndex, owner);
  }

  public EventPointer? PrevInternal(in EventPointer current)
  {
    if (current.IsInInsertedMap)
    {
      if (current.IsInFirstEvents)
      {
        Debug.Assert(insertedEvents.FirstEvents is { });
        if (current.IndexInInsertionMap == 0) return null;

        return EventPointer.ForFirstEvent(current.IndexInInsertionMap - 1, owner);
      }

      if (current.IndexInInsertionMap == 0)
      {
        return EventPointer.ForInitialArray(current.IndexInArray, owner);
      }

      return EventPointer.ForInsertionMap(current.IndexInArray, current.IndexInInsertionMap - 1, owner);
    }

    if (current.IndexInArray == 0)
    {
      if (insertedEvents.FirstEvents is null || insertedEvents.FirstEvents.Count == 0) return null;

      return EventPointer.ForFirstEvent(insertedEvents.FirstEvents.Count - 1, owner);
    }

    if (insertedEvents[current.IndexInArray - 1] is { } insertionList)
    {
      return EventPointer.ForInsertionMap(current.IndexInArray - 1, insertionList.Count - 1, owner);
    }

    return EventPointer.ForInitialArray(current.IndexInArray - 1, owner);
  }

  private List<EventRecordWithMetadata> GetInsertedEventsListOrThrow(EventPointer pointer)
  {
    Debug.Assert(pointer.IsInInsertedMap);
    if (pointer.IsInFirstEvents)
    {
      Debug.Assert(insertedEvents.FirstEvents is { });
      Debug.Assert(pointer.IndexInInsertionMap >= 0 &&
                   pointer.IndexInInsertionMap < insertedEvents.FirstEvents.Count);

      return insertedEvents.FirstEvents;
    }

    Debug.Assert(pointer.IndexInArray >= 0 && pointer.IndexInArray < initialCount);
    Debug.Assert(insertedEvents.ContainsKey(pointer.IndexInArray));
    return insertedEvents[pointer.IndexInArray]!;
  }

  [Conditional("DEBUG")]
  public void AssertStateOrThrow(EventPointer pointer)
  {
    pointer.AssertStateOrThrow();
    if (pointer.IsInInsertedMap)
    {
      if (pointer.IsInFirstEvents)
      {
        Debug.Assert(pointer.IndexInArray == -1);
      }
      else
      {
        Debug.Assert(pointer.IndexInArray >= 0 && pointer.IndexInArray < initialCount);
        Debug.Assert(pointer.IndexInInsertionMap >= 0);
      }
    }
    else
    {
      Debug.Assert(pointer.IsInInitialArray);
      Debug.Assert(pointer.IndexInArray >= 0 && pointer.IndexInArray < initialCount);
      Debug.Assert(pointer.IndexInInsertionMap == -1);
    }
  }
}