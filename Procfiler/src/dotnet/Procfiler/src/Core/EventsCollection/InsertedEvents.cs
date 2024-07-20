using Core.Utils;
using Procfiler.Core.EventRecord;

namespace Procfiler.Core.EventsCollection;

public class InsertedEvents
{
  private readonly Dictionary<int, List<EventRecordWithMetadata>> myInsertedEvents = new();
  private readonly Dictionary<int, HashSet<int>> myRemovedInsertedEvents = new();
  private readonly HashSet<int> myRemovedFromFirstEvents = [];

  public List<EventRecordWithMetadata>? FirstEvents { get; private set; }

  public long Count { get; private set; }


  public EventRecordWithMetadata GetOrThrow(EventPointer pointer) =>
    myInsertedEvents[pointer.IndexInArray][pointer.IndexInInsertionMap];

  public List<EventRecordWithMetadata>? this[int index]
  {
    get => myInsertedEvents.TryGetValue(index, out var insertedEvents) ? insertedEvents : null;
    set => myInsertedEvents[index] = value ?? throw new ArgumentNullException(nameof(value));
  }

  public bool ContainsKey(int index) => myInsertedEvents.ContainsKey(index);

  private List<EventRecordWithMetadata> GetOrCreateInsertedEventsList(EventPointer pointer)
  {
    if (pointer.IsInFirstEvents)
    {
      return FirstEvents ??= [];
    }

    return GetOrCreateInsertionList(pointer.IndexInArray);
  }

  public void InsertAfter(EventPointer pointer, EventRecordWithMetadata eventToInsert)
  {
    if (pointer.IsInInsertedMap)
    {
      GetOrCreateInsertedEventsList(pointer).Insert(pointer.IndexInInsertionMap + 1, eventToInsert);
    }
    else if (pointer.IsInInitialArray)
    {
      Debug.Assert(!myInsertedEvents.ContainsKey(pointer.IndexInArray));
      var insertedEvents = new List<EventRecordWithMetadata> { eventToInsert };
      myInsertedEvents[pointer.IndexInArray] = insertedEvents;
    }

    IncreaseCount();
  }

  private void IncreaseCount() => ++Count;
  private void DecreaseCount() => --Count;

  public void InsertBefore(EventPointer pointer, EventRecordWithMetadata eventToInsert)
  {
    if (pointer.IsInInsertedMap)
    {
      GetOrCreateInsertedEventsList(pointer).Insert(pointer.IndexInInsertionMap, eventToInsert);
    }
    else if (pointer.IsInInitialArray)
    {
      if (pointer.IndexInArray == 0)
      {
        FirstEvents ??= [];
        FirstEvents.Add(eventToInsert);
      }
      else
      {
        GetOrCreateInsertionList(pointer.IndexInArray - 1).Add(eventToInsert);
      }
    }

    IncreaseCount();
  }

  private List<EventRecordWithMetadata> GetOrCreateInsertionList(int index)
  {
    if (myInsertedEvents.TryGetValue(index, out var events)) return events;

    var newList = new List<EventRecordWithMetadata>();
    myInsertedEvents[index] = newList;
    return newList;
  }

  public bool Remove(EventPointer pointer)
  {
    Debug.Assert(!pointer.IsInInitialArray);
    Debug.Assert(pointer.IsInFirstEvents || pointer.IsInInsertedMap);

    if (pointer.IsInFirstEvents)
    {
      if (myRemovedFromFirstEvents.Contains(pointer.IndexInInsertionMap)) return false;

      myRemovedFromFirstEvents.Add(pointer.IndexInInsertionMap);
      DecreaseCount();
      return true;
    }

    var removed = myRemovedInsertedEvents.GetOrCreate(pointer.IndexInArray, static () => []);
    if (removed.Contains(pointer.IndexInInsertionMap))
    {
      return false;
    }

    removed.Add(pointer.IndexInInsertionMap);
    DecreaseCount();
    return true;
  }

  public bool IsRemoved(EventPointer pointer)
  {
    Debug.Assert(!pointer.IsInInitialArray);
    Debug.Assert(pointer.IsInFirstEvents || pointer.IsInInsertedMap);

    if (pointer.IsInFirstEvents)
    {
      return myRemovedFromFirstEvents.Contains(pointer.IndexInInsertionMap);
    }

    var removed = DictionaryExtensions.GetValueOrDefault(myRemovedInsertedEvents, pointer.IndexInArray);
    return removed?.Contains(pointer.IndexInInsertionMap) ?? false;
  }
}