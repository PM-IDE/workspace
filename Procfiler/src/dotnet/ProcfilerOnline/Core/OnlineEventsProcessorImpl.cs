using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Diagnostics.Tracing;
using Microsoft.Diagnostics.Tracing.Parsers;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Commands;
using ProcfilerOnline.Core.Mutators;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core;

public interface IOnlineEventsProcessor
{
  ISharedEventPipeStreamData Process(Stream eventPipeStream, CollectEventsOnlineContext commandContext);
}

[AppComponent]
public class OnlineEventsProcessorImpl(
  IProcfilerLogger logger,
  IThreadsMethodsProcessor methodsProcessor,
  IAppExitHandler appExitHandler,
  IMethodBeginEndSingleMutator methodBeginEndSingleMutator
) : IOnlineEventsProcessor
{
  public ISharedEventPipeStreamData Process(Stream eventPipeStream, CollectEventsOnlineContext commandContext)
  {
    using var source = new EventPipeEventSource(eventPipeStream);

    var globalData = new SharedEventPipeStreamData();

    SubscribeToEventSource(globalData, commandContext, source);

    source.Process();

    ProcessNotClosedMethods(globalData, commandContext);

    appExitHandler.PerformExitActions();

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
    var eventRecord = new EventRecordWithMetadata(traceEvent, -1, traceEvent.ThreadID, -1);

    var context = new EventProcessingContext
    {
      TraceEvent = traceEvent,
      SharedData = globalData,
      Event = eventRecord,
      CommandContext = new CommandContext
      {
        ApplicationName = commandContext.ApplicationName,
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

        var methodEvent = method.ConvertToMethodEndEvent(globalData, methodBeginEndSingleMutator);
        var context = new EventProcessingContext
        {
          TraceEvent = null,
          SharedData = globalData,
          Event = methodEvent,
          CommandContext = new CommandContext
          {
            ApplicationName = commandContext.ApplicationName,
            TargetMethodsRegex = commandContext.TargetMethodsRegex
          }
        };

        ProcessEventInternal(context);
      }
    }
  }
}