using Core.Container;
using Core.Events.EventRecord;
using Core.EventsProcessing.Mutators.Core;
using Core.Utils;
using Dia2Lib;
using Microsoft.Diagnostics.Tracing;
using Microsoft.Diagnostics.Tracing.Parsers;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Commands;
using ProcfilerOnline.Core.Processors;
using ProcfilerOnline.Core.Statistics;
using ProcfilerOnline.Core.Updaters;

namespace ProcfilerOnline.Core;

public interface IOnlineEventsProcessor
{
  ISharedEventPipeStreamData Process(Stream eventPipeStream, CollectEventsOnlineContext commandContext);
}

public interface IEventProcessingEntryPoint
{
  void Process(EventProcessingContext context);
}

[AppComponent]
public class EventProcessingEntryPoint(
  IEnumerable<ITraceEventProcessor> processors,
  IEnumerable<ISingleEventMutator> mutators,
  IStatisticsManager statisticsManager
) : IEventProcessingEntryPoint
{
  private readonly IReadOnlyList<ISingleEventMutator> myOrderedSingleMutators =
    mutators.OrderBy(mutator => mutator.GetPassOrThrow()).ToList();

  public void Process(EventProcessingContext context)
  {
    foreach (var sharedDataUpdater in processors.OfType<ISharedDataUpdater>())
    {
      sharedDataUpdater.Process(context);
    }

    foreach (var mutator in myOrderedSingleMutators)
    {
      mutator.Process(context.Event, context.SharedData);
    }

    statisticsManager.UpdateProcessedEventStatistics(context.Event);
    foreach (var processor in processors.Where(p => p is not ISharedDataUpdater))
    {
      processor.Process(context);
    }
  }
}

[AppComponent]
public class OnlineEventsProcessorImpl(
  IProcfilerLogger logger,
  IStatisticsManager statisticsManager,
  IThreadsMethodsProcessor methodsProcessor
) : IOnlineEventsProcessor
{
  public ISharedEventPipeStreamData Process(Stream eventPipeStream, CollectEventsOnlineContext commandContext)
  {
    using var source = new EventPipeEventSource(eventPipeStream);

    var globalData = new SharedEventPipeStreamData();

    SubscribeToEventSource(globalData, commandContext, source);

    source.Process();

    ProcessNotClosedMethods(globalData, commandContext);

    statisticsManager.Log(logger);

    return globalData;
  }

  private void SubscribeToEventSource(
    ISharedEventPipeStreamData globalData, CollectEventsOnlineContext commandContext, EventPipeEventSource source)
  {
    new TplEtwProviderTraceEventParser(source).All += e => ProcessEvent(e, globalData, commandContext);
    source.Clr.All += e => ProcessEvent(e, globalData, commandContext);
    source.Dynamic.All += e => ProcessEvent(e, globalData, commandContext);
  }

  private void ProcessEvent(TraceEvent traceEvent, ISharedEventPipeStreamData globalData, CollectEventsOnlineContext commandContext)
  {
    var eventRecord = new EventRecordWithMetadata(traceEvent, traceEvent.ThreadID, -1);

    var context = new EventProcessingContext
    {
      TraceEvent = traceEvent,
      SharedData = globalData,
      Event = eventRecord,
      CommandContext = new CommandContext
      {
        TargetMethodsRegex = commandContext.TargetMethodsRegex
      }
    };

    ProcessEventInternal(context);
  }

  private void ProcessEventInternal(EventProcessingContext context) => methodsProcessor.Process(context);

  private void ProcessNotClosedMethods(ISharedEventPipeStreamData globalData, CollectEventsOnlineContext commandContext)
  {
    foreach (var (threadId, methodEvents) in methodsProcessor.ReclaimNotClosedMethods())
    {
      logger.LogWarning("Processing not closed methods for thread {ThreadId}", threadId);
      foreach (var method in methodEvents)
      {
        logger.LogWarning("Processing method-event {EventName}", method.EventName);

        var methodEvent = method.ConvertToMethodEndEvent();
        var context = new EventProcessingContext
        {
          TraceEvent = null,
          SharedData = globalData,
          Event = methodEvent,
          CommandContext = new CommandContext
          {
            TargetMethodsRegex = commandContext.TargetMethodsRegex
          }
        };

        ProcessEventInternal(context);
      }
    }
  }
}