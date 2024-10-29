using Core.Events.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection;
using ProcfilerTests.Core;
using TestsUtil;

namespace ProcfilerTests.Tests.EventsCollection;

[TestFixture]
public class EventsCollectionTests
{
  public readonly record struct TestData(IEventsCollection Collection, EventRecordWithMetadata[] Events);

  private static TestData[] NonEmptyCollections
  {
    get
    {
      var lengthOneEvents = CreateInitialArrayOfRandomEvents(count: 1);

      var logger = TestLogger.CreateInstance();

      List<TestData> data =
      [
        new(new EventsCollectionImpl(lengthOneEvents, logger), lengthOneEvents),
      ];

      const int CollectionsCount = 10;
      for (var i = 0; i < CollectionsCount; ++i)
      {
        var events = CreateInitialArrayOfRandomEvents();
        data.Add(new TestData(new EventsCollectionImpl(events, logger), events));
      }

      return data.ToArray();
    }
  }

  private static TestData[] Collections
  {
    get
    {
      EventRecordWithMetadata[] empty = [];

      List<TestData> data =
      [
        new(new EventsCollectionImpl(empty, TestLogger.CreateInstance()), empty),
      ];

      data.AddRange(NonEmptyCollections);

      return data.ToArray();
    }
  }


  [Test]
  [TestCaseSource(nameof(Collections))]
  public void TestEnumeration(TestData testData)
  {
    var (collection, events) = testData;

    Assert.That(collection, Has.Count.EqualTo(events.Length));
    foreach (var (arrayEvent, (_, collectionEvent)) in events.Zip(collection))
    {
      Assert.That(ReferenceEquals(arrayEvent, collectionEvent), Is.True);
    }
  }

  [Test]
  [TestCaseSource(nameof(NonEmptyCollections))]
  public void TestInsertionInBeginning(TestData testData)
  {
    var (collection, events) = testData;

    var eventsToInsert = CreateInitialArrayOfRandomEvents(10);

    collection.ApplyNotPureActionForAllEvents(eventWithPtr =>
    {
      foreach (var eventRecord in eventsToInsert)
      {
        collection.InsertBefore(eventWithPtr.EventPointer, eventRecord);
      }

      return true;
    });

    Assert.That(collection, Has.Count.EqualTo(events.Length + eventsToInsert.Length));
    AssertCollectionsAreSame(collection, eventsToInsert.Concat(events));
  }

  [Test]
  [TestCaseSource(nameof(NonEmptyCollections))]
  public void TestInsertBefore(TestData testData)
  {
    var (collection, events) = testData;

    var index = Random.Shared.Next((int)collection.Count);
    var eventRecord = TestUtil.CreateAbsolutelyRandomEvent();
    var pointerForIndex = EventPointer.ForInitialArray(index, collection);
    var eventRecordAtIndex = SlowlyFindEventFor(collection, pointerForIndex);
    Assert.That(eventRecordAtIndex, Is.Not.Null);

    collection.InsertBefore(EventPointer.ForInitialArray(index, collection), eventRecord);

    var eventRecordAtIndexAfterInsertion = SlowlyFindEventFor(collection, pointerForIndex);
    Assert.That(ReferenceEquals(eventRecordAtIndex, eventRecordAtIndexAfterInsertion), Is.True);
    Assert.That(collection, Has.Count.EqualTo(events.Length + 1));

    var list = events.ToList();
    list.Insert(index, eventRecord);

    AssertCollectionsAreSame(collection, list);
  }

  private static EventRecordWithMetadata? SlowlyFindEventFor(IEventsCollection collection, EventPointer pointer)
  {
    EventRecordWithMetadata? eventRecordAtIndex = null;
    collection.ApplyNotPureActionForAllEvents(eventWithPtr =>
    {
      if (eventWithPtr.EventPointer == pointer)
      {
        eventRecordAtIndex = eventWithPtr.Event;
        return true;
      }

      return false;
    });

    return eventRecordAtIndex;
  }

  [Test]
  [TestCaseSource(nameof(NonEmptyCollections))]
  public void TestInsertAfter(TestData testData)
  {
    var (collection, events) = testData;

    var index = Random.Shared.Next((int)collection.Count);
    var eventRecord = TestUtil.CreateAbsolutelyRandomEvent();
    var pointerForIndex = EventPointer.ForInitialArray(index, collection);
    var eventRecordAtIndex = SlowlyFindEventFor(collection, pointerForIndex);
    Assert.That(eventRecordAtIndex, Is.Not.Null);

    collection.InsertAfter(EventPointer.ForInitialArray(index, collection), eventRecord);

    var eventRecordAtIndexAfterInsertion = SlowlyFindEventFor(collection, pointerForIndex);
    Assert.That(ReferenceEquals(eventRecordAtIndex, eventRecordAtIndexAfterInsertion), Is.True);
    Assert.That(collection, Has.Count.EqualTo(events.Length + 1));

    var list = events.ToList();
    list.Insert(index + 1, eventRecord);

    AssertCollectionsAreSame(collection, list);
  }

  private static void AssertCollectionsAreSame(
    IEventsCollection collection, IEnumerable<EventRecordWithMetadata> events)
  {
    foreach (var ((_, collectionEvent), listEvent) in collection.Zip(events))
    {
      Assert.That(ReferenceEquals(collectionEvent, listEvent), Is.True);
    }
  }

  [Test]
  [TestCaseSource(nameof(NonEmptyCollections))]
  public void TestRemoval(TestData testData)
  {
    var (collection, events) = testData;

    var index = Random.Shared.Next((int)collection.Count);

    collection.Remove(EventPointer.ForInitialArray(index, collection));

    Assert.That(collection, Has.Count.EqualTo(events.Length - 1));
    var list = events.ToList();
    list.RemoveAt(index);

    AssertCollectionsAreSame(collection, list);
  }

  [Test]
  [TestCaseSource(nameof(Collections))]
  public void TestMultipleInsertAfter(TestData testData)
  {
    var (collection, events) = testData;

    var eventsToInsert = CreateInitialArrayOfRandomEvents();

    var index = 0;
    var added = false;
    collection.ApplyNotPureActionForAllEvents(eventWithPtr =>
    {
      if (!added)
      {
        added = true;
        collection.InsertAfter(eventWithPtr.EventPointer, eventsToInsert[index++]);
      }
      else
      {
        added = false;
      }

      return false;
    });

    var expectedResult = new List<EventRecordWithMetadata>();
    foreach (var (initialEvent, insertedEvent) in events.Zip(eventsToInsert))
    {
      expectedResult.Add(initialEvent);
      expectedResult.Add(insertedEvent);
    }

    AssertCollectionsAreSame(collection, expectedResult);
  }

  [Test]
  [TestCaseSource(nameof(Collections))]
  public void TestMultipleInsertBefore(TestData testData)
  {
    var (collection, events) = testData;

    var eventsToInsert = CreateInitialArrayOfRandomEvents();

    var index = 0;
    collection.ApplyNotPureActionForAllEvents(eventWithPtr =>
    {
      collection.InsertBefore(eventWithPtr.EventPointer, eventsToInsert[index++]);
      return false;
    });

    var expectedResult = new List<EventRecordWithMetadata>();
    foreach (var (insertedEvent, initialEvent) in eventsToInsert.Zip(events))
    {
      expectedResult.Add(insertedEvent);
      expectedResult.Add(initialEvent);
    }

    AssertCollectionsAreSame(collection, expectedResult);
  }

  [Test]
  public void TestFirst()
  {
    var events = CreateInitialArrayOfRandomEvents();
    var collection = CreateNewCollection(events);

    Assert.Multiple(() =>
    {
      Assert.That(collection, Has.Count.EqualTo(events.Length));
      Assert.That(ReferenceEquals(GetFirstEvent(collection)?.Event, events[0]), Is.True);
    });
  }

  private static EventRecordWithPointer? GetFirstEvent(IEventsCollection collection)
  {
    EventRecordWithPointer? eventRecordAtIndex = null;
    collection.ApplyNotPureActionForAllEvents(eventWithPtr =>
    {
      eventRecordAtIndex = eventWithPtr;
      return true;
    });

    return eventRecordAtIndex;
  }


  [Test]
  [TestCaseSource(nameof(NonEmptyCollections))]
  public void TestLast(TestData testData)
  {
    var (collection, events) = testData;


    Assert.That(collection, Has.Count.EqualTo(events.Length));
    Assert.That(ReferenceEquals(SlowlyGetLastEvent(collection)?.Event, events[^1]), Is.True);
  }

  private static EventRecordWithPointer? SlowlyGetLastEvent(IEventsCollection collection)
  {
    EventRecordWithPointer? eventRecordAtIndex = null;
    collection.ApplyNotPureActionForAllEvents(eventWithPtr =>
    {
      eventRecordAtIndex = eventWithPtr;
      return false;
    });

    return eventRecordAtIndex;
  }

  [Test]
  [TestCaseSource(nameof(NonEmptyCollections))]
  public void TestFreezeOfCollections(TestData testData)
  {
    var (collection, _) = testData;

    var randomEvent = TestUtil.CreateAbsolutelyRandomEvent();
    var actions = new TestDelegate[]
    {
      () => collection.InsertAfter(GetFirstEvent(collection)!.Value.EventPointer, randomEvent),
      () => collection.InsertBefore(SlowlyGetLastEvent(collection)!.Value.EventPointer, randomEvent),
      () => collection.Remove(GetFirstEvent(collection)!.Value.EventPointer)
    };

    collection.Freeze();
    foreach (var action in actions)
    {
      Assert.Throws<CollectionIsFrozenException>(action);
    }

    collection.UnFreeze();
    foreach (var action in actions)
    {
      Assert.DoesNotThrow(action);
    }
  }

  [Test]
  public void TestInsertBeforeFirst()
  {
    var events = CreateInitialArrayOfRandomEvents(1);
    var collection = CreateNewCollection(events);

    Assert.That(collection, Has.Count.EqualTo(1));
    var eventToInsert = TestUtil.CreateAbsolutelyRandomEvent();
    collection.InsertBefore(GetFirstEvent(collection)!.Value.EventPointer, eventToInsert);
    Assert.That(collection, Has.Count.EqualTo(2));
    AssertCollectionsAreSame(collection, new[] { eventToInsert }.Concat(events));
  }

  [Test]
  public void TestInsertAfterLast()
  {
    var events = CreateInitialArrayOfRandomEvents(1);
    var collection = CreateNewCollection(events);

    Assert.That(collection, Has.Count.EqualTo(1));
    var eventToInsert = TestUtil.CreateAbsolutelyRandomEvent();
    collection.InsertAfter(GetFirstEvent(collection)!.Value.EventPointer, eventToInsert);
    Assert.That(collection, Has.Count.EqualTo(2));
    AssertCollectionsAreSame(collection, events.Concat([eventToInsert]));
  }

  private static IEventsCollection CreateNewCollection(EventRecordWithMetadata[] events) =>
    new EventsCollectionImpl(events, TestLogger.CreateInstance());

  private static EventRecordWithMetadata[] CreateInitialArrayOfRandomEvents(int count = 200) =>
    Enumerable.Range(0, count).Select(_ => TestUtil.CreateAbsolutelyRandomEvent()).OrderBy(e => e.Time.QpcStamp).ToArray();

  [Test]
  [TestCaseSource(nameof(Collections))]
  public void TestEnumerationWithModificationSource(TestData testData)
  {
    var (collection, events) = testData;

    var modificationSourceEvents = CreateInitialArrayOfRandomEvents();
    var modificationSource = new TestModificationSource(TestLogger.CreateInstance(), modificationSourceEvents);
    collection.InjectModificationSource(modificationSource);

    var concatenation = events.Concat(modificationSourceEvents).OrderBy(x => x.Time.QpcStamp);

    AssertCollectionsAreSame(collection, concatenation);
  }

  [Test]
  [TestCaseSource(nameof(Collections))]
  public void TestCollectionCountAfterModificationInjection(TestData testData)
  {
    var (collection, events) = testData;

    const int ModificationSourcesCount = 10;

    var additionalLength = 0;
    for (var i = 0; i < ModificationSourcesCount; i++)
    {
      var modificationEvents = CreateInitialArrayOfRandomEvents();
      additionalLength += modificationEvents.Length;
      collection.InjectModificationSource(new TestModificationSource(TestLogger.CreateInstance(), modificationEvents));
    }

    Assert.That(collection.Count, Is.EqualTo(events.Length + additionalLength));
  }

  [Test]
  [TestCaseSource(nameof(Collections))]
  public void TestEnumerationWithManyModificationSources(TestData testData)
  {
    var (collection, events) = testData;

    const int ModificationSourcesCount = 10;

    var modifications = new List<EventRecordWithMetadata[]>();
    for (var i = 0; i < ModificationSourcesCount; i++)
    {
      var modificationEvents = CreateInitialArrayOfRandomEvents();
      modifications.Add(modificationEvents);
      collection.InjectModificationSource(new TestModificationSource(TestLogger.CreateInstance(), modificationEvents));
    }

    var concatenation = events.Concat(modifications.SelectMany(source => source)).OrderBy(e => e.Time.QpcStamp);
    AssertCollectionsAreSame(collection, concatenation);
  }

  [Test]
  public void TestEmptyCollection()
  {
    var collection = CreateNewCollection([]);
    Assert.That(collection, Is.Empty);
  }
}