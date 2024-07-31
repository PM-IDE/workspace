using Core.Container;
using Core.Events.EventRecord;
using Core.Methods;

namespace ProcfilerOnline.Core.Processors;

[AppComponent]
public class AsyncMethodsProcessor : ITraceEventProcessor
{
  private readonly OnlineAsyncMethodsGrouper<EventRecordWithMetadata> myGrouper = new("ASYNC_", HandleAsyncMethod);


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
      if (context.Event.IsTaskWaitSendEvent(out var sendTaskId))
      {
        lastSeenTaskEvent = new TaskWaitSendEvent { TaskId = sendTaskId };
      }

      if (context.Event.IsTaskWaitStopEvent(out var waitTaskId))
      {
        lastSeenTaskEvent = new TaskWaitStopEvent { TaskId = waitTaskId };
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
      foreach (var frame in trace)
      {
        Console.WriteLine(frame.EventName);
      }

      Console.WriteLine();
    }
  }
}