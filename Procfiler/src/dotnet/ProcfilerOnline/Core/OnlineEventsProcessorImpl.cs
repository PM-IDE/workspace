using Core.Container;
using Core.Events.EventRecord;
using Core.EventsProcessing.Mutators.Core;
using Core.Utils;
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

[AppComponent]
public class OnlineEventsProcessorImpl(
  IProcfilerLogger logger,
  IEnumerable<ITraceEventProcessor> processors,
  IEnumerable<ISingleEventMutator> singleEventMutators,
  IStatisticsManager statisticsManager,
  IThreadsMethodsProcessor methodsProcessor
) : IOnlineEventsProcessor
{
  private readonly IReadOnlyList<ISingleEventMutator> myOrderedSingleMutators =
    singleEventMutators.OrderBy(mutator => mutator.GetPassOrThrow()).ToList();

  public ISharedEventPipeStreamData Process(Stream eventPipeStream, CollectEventsOnlineContext commandContext)
  {
    using var source = new EventPipeEventSource(eventPipeStream);

    var globalData = new SharedEventPipeStreamData();

    new TplEtwProviderTraceEventParser(source).All += e => ProcessEvent(e, globalData, commandContext);
    source.Clr.All += e => ProcessEvent(e, globalData, commandContext);
    source.Dynamic.All += e => ProcessEvent(e, globalData, commandContext);

    source.Process();

    statisticsManager.Log(logger);

    foreach (var (threadId, methodEvents) in methodsProcessor.ReclaimNotClosedMethods())
    {
      logger.LogWarning("Processing not closed methods for thread {ThreadId}", threadId);
      foreach (var method in methodEvents)
      {
        logger.LogWarning("Processing method-event {EventName}", method.EventName);

        var methodEvent = method.DeepClone();
        methodEvent.EventClass = OnlineProcfilerConstants.CppMethodFinishedEventName;

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

        ProcessEvent(context);
      }
    }

    return globalData;
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

    foreach (var sharedDataUpdater in processors.OfType<ISharedDataUpdater>())
    {
      sharedDataUpdater.Process(context);
    }

    ProcessEvent(context);
  }

  private void ProcessEvent(EventProcessingContext context)
  {
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