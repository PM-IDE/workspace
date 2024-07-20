using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsCollection;
using Procfiler.Core.EventsProcessing;
using Procfiler.Core.EventsProcessing.Mutators;
using Procfiler.Core.Serialization.Core;
using Procfiler.Utils;

namespace Procfiler.Commands.CollectClrEvents.Base;

public abstract class CollectAndSplitCommandBase<TKey>(
  IProcfilerLogger logger,
  ICommandExecutorDependantOnContext commandExecutor,
  IUndefinedThreadsEventsMerger undefinedThreadsEventsMerger,
  IUnitedEventsProcessor unitedEventsProcessor,
  IDelegatingEventsSerializer delegatingEventsSerializer,
  IStackTraceSerializer stackTraceSerializer
) : CollectCommandBase(logger, commandExecutor) where TKey : notnull
{
  protected record struct CollectAndSplitContext(bool UseFilters, bool UseMutators, bool MergeFromUndefinedThread)
  {
    public static CollectAndSplitContext DoNothing { get; } = new(false, false, false);
    public static CollectAndSplitContext DoEverything { get; } = new(true, true, true);
  }

  protected readonly IUnitedEventsProcessor UnitedEventsProcessor = unitedEventsProcessor;


  protected void ExecuteSimpleSplitCommand(
    CollectClrEventsContext context,
    Func<EventRecordWithMetadata, TKey> keyExtractor,
    CollectAndSplitContext collectAndSplitContext)
  {
    ExecuteCommand(context, collectedEvents =>
    {
      PathUtils.CheckIfDirectoryOrThrow(context.CommonContext.OutputPath);

      var (allEvents, globalData) = collectedEvents;
      var (useFilters, useMutators, mergeFromUndefinedThread) = collectAndSplitContext;
      var processingContext = CreateContext(allEvents, globalData, useFilters, useMutators, false);
      UnitedEventsProcessor.ProcessFullEventLog(processingContext);

      var eventsByKey = SplitEventsHelper.SplitByKey(Logger, allEvents, keyExtractor);
      var undefinedEventsPair = FindEventsForUndefinedThread(eventsByKey);

      var undefinedEvents = undefinedEventsPair?.Value;
      undefinedEvents = AddNewManagedThreadsFromUndefined(eventsByKey, globalData, undefinedEvents);

      if (!mergeFromUndefinedThread)
      {
        undefinedEvents = null;
      }
      else if (undefinedEvents is { } && undefinedEventsPair is { Key: { } key })
      {
        eventsByKey.Remove(key);
      }

      foreach (var (key, managedEvents) in eventsByKey)
      {
        ProcessEvents(managedEvents, undefinedEvents, context, key, globalData, collectAndSplitContext);
      }

      SerializeStacks(context, globalData);
    });
  }

  private void ProcessEvents(
    IEventsCollection correspondingEvents,
    IEventsCollection? undefinedThreadEvents,
    CollectClrEventsContext context,
    TKey key,
    SessionGlobalData globalData,
    CollectAndSplitContext collectAndSplitContext)
  {
    if (collectAndSplitContext.UseMutators)
    {
      UnitedEventsProcessor.ApplyMultipleMutators(correspondingEvents, globalData, EmptyCollections<Type>.EmptySet);
    }

    if (correspondingEvents.Count == 0) return;

    IEnumerable<EventRecordWithPointer> mergedEvents = correspondingEvents;
    if (undefinedThreadEvents is { })
    {
      using (new PerformanceCookie($"{GetType().Name}::MergingEvents", Logger))
      {
        mergedEvents = undefinedThreadsEventsMerger.MergeLazy(correspondingEvents, undefinedThreadEvents);
      }
    }

    var outputFormat = context.CommonContext.SerializationContext.OutputFormat;
    var extension = outputFormat.GetExtension();
    var filePath = Path.Combine(context.CommonContext.OutputPath, $"{key.ToString()}.{extension}");

    delegatingEventsSerializer.SerializeEvents(mergedEvents.Select(ptr => ptr.Event), filePath, outputFormat);
  }

  private void SerializeStacks(CollectClrEventsContext context, SessionGlobalData globalData)
  {
    stackTraceSerializer.SerializeStackTraces(globalData, context.CommonContext.OutputPath);
  }

  private static EventsProcessingContext CreateContext(
    IEventsCollection currentEvents,
    SessionGlobalData globalData,
    bool useFilters,
    bool useMutators,
    bool useMethodStartEndMutator)
  {
    var config = useMethodStartEndMutator switch
    {
      false => EventsProcessingConfig.CreateContextWithoutStartAndEnd(useFilters, useMutators),
      true => new EventsProcessingConfig(useFilters, useMutators, EmptyCollections<Type>.EmptySet)
    };

    return new EventsProcessingContext(currentEvents, globalData, config);
  }

  protected virtual KeyValuePair<TKey, IEventsCollection?>? FindEventsForUndefinedThread(
    Dictionary<TKey, IEventsCollection> eventsByKey) =>
    null;

  protected virtual IEventsCollection? AddNewManagedThreadsFromUndefined(
    IDictionary<TKey, IEventsCollection> events,
    SessionGlobalData globalData,
    IEventsCollection? undefinedEvents) =>
    undefinedEvents;
}