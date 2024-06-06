using Autofac;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsCollection;
using Procfiler.Core.EventsProcessing;
using Procfiler.Core.Serialization.MethodsCallTree;
using Procfiler.Utils;

namespace ProcfilerTests.Core;

public static class TestUtil
{
  public static IEventsCollection FindEventsForMainThread(IEventsCollection events)
  {
    var eventsByThreads = SplitEventsHelper.SplitByKey(
      TestLogger.CreateInstance(), events, SplitEventsHelper.ManagedThreadIdExtractor);

    return FindEventsForMainThread(eventsByThreads);
  }

  public static IEventsCollection FindEventsForMainThread(
    Dictionary<long, IEventsCollection> eventsByThreads)
  {
    var (_, mainThreadEvents) = eventsByThreads.Where(e => e.Key != -1).MaxBy(e => e.Value.Count);
    return mainThreadEvents;
  }

  public static EventsProcessingContext CreateEventsProcessingContext(
    IEventsCollection managedThreadEvents,
    SessionGlobalData globalData)
  {
    var config = new EventsProcessingConfig(false, true, EmptyCollections<Type>.EmptySet);
    return new EventsProcessingContext(managedThreadEvents, globalData, config);
  }

  public static void CheckMethodConsistencyOrThrow(
    long threadId, IEventsCollection events, SessionGlobalData globalData, IContainer container)
  {
    void AssertFail(string message)
    {
      SerializeBrokenStacks(container, globalData, threadId, events);
      Assert.Fail(message);
    }

    var frames = new Stack<string>();
    var index = -1;
    string? assertMessage = null;

    foreach (var (_, eventRecord) in events)
    {
      ++index;
      if (eventRecord.TryGetMethodStartEndEventInfo() is var (frame, isStart))
      {
        if (isStart)
        {
          frames.Push(frame);
        }
        else
        {
          if (frames.Count == 0)
          {
            assertMessage = $"Stack was empty, index = {index}, ts = {eventRecord.Time.QpcStamp}";
            continue;
          }

          var topMost = frames.Pop();
          if (topMost != frame)
          {
            assertMessage = $"{topMost} != {frame}, index = {index}, ts = {eventRecord.Time.QpcStamp}";
            break;
          }
        }
      }
    }

    if (assertMessage is { })
    {
      AssertFail(assertMessage);
    }

    if (frames.Count != 0)
    {
      AssertFail("frames.Count != 0");
    }
  }

  private static void SerializeBrokenStacks(
    IContainer container,
    SessionGlobalData globalData,
    long brokenThreadId,
    IEventsCollection eventsCollection)
  {
    var savePath = PathUtils.CreateTempFolderPath();
    var serializer = container.Resolve<IStackTraceSerializer>();
    Console.WriteLine($"Serializing broken stacks at {savePath}, thread ID {brokenThreadId}");

    serializer.SerializeStackTraces(globalData, savePath);

    var eventsSerializer = container.Resolve<IMethodTreeEventSerializer>();
    eventsSerializer.SerializeEvents(eventsCollection.Select(ptr => ptr.Event), Path.Combine(savePath, "events.txt"));
  }

  public static EventRecordWithMetadata CreateRandomEvent(string eventClass, EventMetadata metadata)
  {
    var randomStamp = EventRecordTime.QpcOnly(Random.Shared.NextInt64(long.MaxValue));
    var randomManagedThreadId = Random.Shared.Next(10) - 1;
    return new EventRecordWithMetadata(randomStamp, eventClass, randomManagedThreadId, -1, metadata);
  }

  public static EventRecordWithMetadata CreateAbsolutelyRandomEvent()
  {
    var randomStamp = Random.Shared.NextInt64(long.MaxValue);
    var randomManagedThreadId = Random.Shared.Next(10) - 1;
    return new EventRecordWithMetadata(
      EventRecordTime.QpcOnly(randomStamp), CreateRandomEventClass(), randomManagedThreadId, -1, new EventMetadata());
  }

  public static string CreateRandomEventClass()
  {
    var chars = Enumerable
      .Range(0, Random.Shared.Next(1, 13))
      .Select(_ => (char)Random.Shared.Next('a', 'z' + 1))
      .ToArray();

    return new string(chars);
  }

  public static void AssertCollectionsAreSame<TValue>(ICollection<TValue> first, ICollection<TValue> second)
  {
    Assert.That(first.Count, Is.EqualTo(second.Count));
    foreach (var (firstItem, secondItem) in first.Zip(second))
    {
      Assert.That(firstItem is { }, Is.True);
      Assert.That(secondItem is { }, Is.True);

      Assert.That(firstItem!.Equals(secondItem), Is.True);
    }
  }
}