using Procfiler.Core.EventsCollection;

namespace Procfiler.Core;

public class OrderedEventsEnumerator : IEnumerable<EventRecordWithPointer>, IEnumerator<EventRecordWithPointer>
{
  private readonly IEnumerator<EventRecordWithPointer>[] myEnumerators;
  private readonly IEnumerable<EventRecordWithPointer>[] myCollections;
  private readonly PriorityQueue<int, long> myQueue;

  private int myLastReturnedEnumerator = -1;


  public EventRecordWithPointer Current { get; private set; }
  object IEnumerator.Current => Current;


  /// <invariant name = "EventsMustBeOrderedByStamp">
  /// The Lists of events in eventsByManagedThreads should be sorted by "Stamp" property
  /// </invariant>
  public OrderedEventsEnumerator(IEnumerable<IEnumerable<EventRecordWithPointer>> eventsByTraces)
  {
    //reference to invariant: EventsMustBeOrderedByStamp
    myCollections = eventsByTraces.ToArray();
    myEnumerators = new IEnumerator<EventRecordWithPointer>[myCollections.Length];
    myQueue = new PriorityQueue<int, long>();

    Reset();
  }


  public bool MoveNext()
  {
    if (myLastReturnedEnumerator != -1)
    {
      ref var enumerator = ref myEnumerators[myLastReturnedEnumerator];
      if (enumerator.MoveNext())
      {
        myQueue.Enqueue(myLastReturnedEnumerator, enumerator.Current.Event.Time.QpcStamp);
      }
    }

    while (true)
    {
      if (myQueue.Count == 0) return false;

      var next = myQueue.Dequeue();
      ref var enumerator = ref myEnumerators[next];

      Current = enumerator.Current;
      myLastReturnedEnumerator = next;

      return true;
    }
  }

  public void Reset()
  {
    myQueue.Clear();
    for (var i = 0; i < myCollections.Length; i++)
    {
      myEnumerators[i] = myCollections[i].GetEnumerator();
    }

    for (var i = 0; i < myCollections.Length; i++)
    {
      ref var enumerator = ref myEnumerators[i];
      if (enumerator.MoveNext())
      {
        myQueue.Enqueue(i, enumerator.Current.Event.Time.QpcStamp);
      }
    }
  }

  public IEnumerator<EventRecordWithPointer> GetEnumerator() => this;

  IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

  public void Dispose()
  {
  }
}