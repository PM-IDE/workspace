﻿using Core.Container;
using Core.Events.EventRecord;
using Core.Methods;
using Core.Utils;

namespace ProcfilerOnline.Core.Processors;

[AppComponent]
public class AsyncMethodsProcessor(IProcfilerLogger logger) : ITraceEventProcessor
{
  private readonly OnlineAsyncMethodsGrouper<EventRecordWithMetadata> myGrouper = new(logger, "ASYNC_", HandleAsyncMethod);


  public void Process(EventProcessingContext context)
  {
    var threadId = context.Event.ManagedThreadId;

    if (context.Event.TryGetMethodDetails() is var (_, methodId))
    {
      if (!context.SharedData.MethodIdToFqn.TryGetValue(methodId, out var fqn)) return;

      if (context.CommandContext.TargetMethodsRegex is null ||
          context.CommandContext.TargetMethodsRegex.IsMatch(fqn))
      {
        myGrouper.ProcessMethodStartEndEvent(context.Event, fqn, context.Event.GetMethodEventKind() == MethodKind.Begin, threadId);
      }

      return;
    }

    if (context.Event.IsTaskWaitSendOrStopEvent())
    {
      LastSeenTaskEvent lastSeenTaskEvent = null!;
      if (context.Event.IsTaskWaitSendEvent(out var sendTaskId, out var originatingTaskId))
      {
        lastSeenTaskEvent = new TaskWaitSendEvent { TaskId = sendTaskId, OriginatingTaskId = originatingTaskId };
      }

      if (context.Event.IsTaskWaitStopEvent(out var waitTaskId, out originatingTaskId))
      {
        lastSeenTaskEvent = new TaskWaitStopEvent { TaskId = waitTaskId, OriginatingTaskId = originatingTaskId };
      }

      myGrouper.ProcessTaskWaitEvent(lastSeenTaskEvent, threadId);
    }

    myGrouper.ProcessNormalEvent(context.Event, threadId);
  }

  private static void HandleAsyncMethod(string stateMachineName, List<List<EventRecordWithMetadata>> traces)
  {
    Console.WriteLine(stateMachineName);
    foreach (var trace in traces)
    {
      Console.WriteLine("Trace start");
      foreach (var eventRecord in trace)
      {
        if (eventRecord.TryGetMethodDetails() is { })
        {
          Console.WriteLine(eventRecord.EventName);
        }
      }

      Console.WriteLine();
    }
  }
}