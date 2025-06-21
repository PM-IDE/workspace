using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord.EventsCollection;
using Procfiler.Core.EventsProcessing;
using Procfiler.Core.EventsProcessing.Mutators;
using Procfiler.Core.Serialization.Core;

namespace Procfiler.Core.SplitByMethod;

public record struct SplitContext(
  CollectedEvents Events,
  string FilterPattern,
  InlineMode InlineMode,
  bool MergeUndefinedThreadEvents,
  bool AddAsyncMethods
);

public interface IByMethodsSplitter
{
  Dictionary<string, List<List<EventRecordWithMetadata>>>? SplitNonAlloc(IOnlineMethodsSerializer serializer,
    SplitContext context);

  Dictionary<string, List<List<EventRecordWithMetadata>>> Split(SplitContext context);
}

[AppComponent]
public class ByMethodsSplitterImpl(
  IProcfilerLogger logger,
  IEventsCollectionByMethodsSplitter splitter,
  IManagedEventsFromUndefinedThreadExtractor managedEventsExtractor,
  IAsyncMethodsGrouper asyncMethodsGrouper,
  IUnitedEventsProcessor unitedEventsProcessor,
  IUndefinedThreadsEventsMerger undefinedThreadsEventsMerger
) : IByMethodsSplitter
{
  public Dictionary<string, List<List<EventRecordWithMetadata>>>? SplitNonAlloc(
    IOnlineMethodsSerializer serializer, SplitContext context)
  {
    var (events, filterPattern, inlineMode, mergeUndefinedThreadEvents, addAsyncMethods) = context;
    SplitEventsByThreads(events, out var eventsByManagedThreads, out var undefinedThreadEvents);

    foreach (var (key, threadEvents) in eventsByManagedThreads)
    {
      using var _ = new PerformanceCookie($"{GetType().Name}::{nameof(Split)}::PreparingTrace_{key}", logger);

      ProcessManagedThreadEvents(threadEvents, events.GlobalData);

      var mergedEvents = mergeUndefinedThreadEvents switch
      {
        true => MergeUndefinedThreadEventsLazy(threadEvents, undefinedThreadEvents),
        false => threadEvents
      };

      serializer.SerializeThreadEvents(mergedEvents, filterPattern, inlineMode);
    }

    if (addAsyncMethods)
    {
      var result = new Dictionary<string, List<List<EventRecordWithMetadata>>>();
      AddAsyncMethods(result, eventsByManagedThreads);

      return result;
    }

    return null;
  }

  public Dictionary<string, List<List<EventRecordWithMetadata>>> Split(SplitContext context)
  {
    var (events, filterPattern, inlineMode, mergeUndefinedThreadEvents, addAsyncMethods) = context;
    SplitEventsByThreads(events, out var eventsByManagedThreads, out var undefinedThreadEvents);

    var tracesByMethods = new Dictionary<string, List<List<EventRecordWithMetadata>>>();
    foreach (var (key, threadEvents) in eventsByManagedThreads)
    {
      using var _ = new PerformanceCookie($"{GetType().Name}::{nameof(Split)}::PreparingTrace_{key}", logger);

      ProcessManagedThreadEvents(threadEvents, events.GlobalData);

      var mergedEvents = mergeUndefinedThreadEvents switch
      {
        true => MergeUndefinedThreadEvents(threadEvents, undefinedThreadEvents),
        false => threadEvents
      };

      var eventsTracesByMethods = splitter.Split(mergedEvents, filterPattern, inlineMode);

      foreach (var (methodName, traces) in eventsTracesByMethods)
      {
        var tracesForMethod = tracesByMethods.GetOrCreate(methodName, static () => []);

        tracesForMethod.AddRange(traces);
      }
    }

    if (addAsyncMethods)
    {
      AddAsyncMethods(tracesByMethods, eventsByManagedThreads);
    }

    return tracesByMethods;
  }

  private void AddAsyncMethods(
    Dictionary<string, List<List<EventRecordWithMetadata>>> tracesByMethods,
    Dictionary<long, IEventsCollection> eventsByManagedThreads)
  {
    var asyncMethodsTraces = asyncMethodsGrouper.GroupAsyncMethods(eventsByManagedThreads, true);
    foreach (var (asyncMethodName, collection) in asyncMethodsTraces)
    {
      var traces = new List<List<EventRecordWithMetadata>>();
      traces.AddRange(collection);

      tracesByMethods[asyncMethodName] = traces;
    }
  }

  private void SplitEventsByThreads(
    CollectedEvents events,
    out Dictionary<long, IEventsCollection> eventsByThreads,
    out IEventsCollection undefinedThreadEvents)
  {
    eventsByThreads = SplitEventsHelper.SplitByKey(logger, events.Events, SplitEventsHelper.ManagedThreadIdExtractor);
    undefinedThreadEvents = eventsByThreads.Remove(-1, out var x) switch
    {
      true => x,
      false => new EventsCollectionImpl(EmptyCollections<EventRecordWithMetadata>.EmptyArray, logger)
    };

    undefinedThreadEvents = managedEventsExtractor.Extract(eventsByThreads, undefinedThreadEvents);
  }

  private void ProcessManagedThreadEvents(IEventsCollection threadEvents, SessionGlobalData globalData)
  {
    using var _ = new PerformanceCookie($"{GetType().Name}::{nameof(ProcessManagedThreadEvents)}", logger);
    unitedEventsProcessor.ApplyMultipleMutators(threadEvents, globalData, EmptyCollections<Type>.EmptySet);
  }

  private IEventsCollection MergeUndefinedThreadEvents(IEventsCollection managedThreadEvents, IEventsCollection undefinedThreadEvents)
  {
    using var __ = new PerformanceCookie($"{GetType().Name}::{nameof(MergeUndefinedThreadEvents)}", logger);
    return undefinedThreadsEventsMerger.Merge(managedThreadEvents, undefinedThreadEvents);
  }

  private IEnumerable<EventRecordWithPointer> MergeUndefinedThreadEventsLazy(IEventsCollection managedThreadEvents,
    IEventsCollection undefinedThreadEvents) => undefinedThreadsEventsMerger.MergeLazy(managedThreadEvents, undefinedThreadEvents);
}