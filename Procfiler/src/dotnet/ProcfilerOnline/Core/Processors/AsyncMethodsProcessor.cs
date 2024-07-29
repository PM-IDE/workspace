using Core.Container;
using Core.Methods;
using Microsoft.Diagnostics.Tracing.Parsers.Tpl;

namespace ProcfilerOnline.Core.Processors;

[AppComponent]
public class AsyncMethodsProcessor(ISharedEventPipeStreamData sharedData) : ITraceEventProcessor
{
  private readonly OnlineAsyncMethodsGrouper<string> myGrouper = new("ASYNC_", HandleAsyncMethod);


  public void Process(EventProcessingContext context)
  {
    var threadId = context.Event.ThreadID;

    if (context.Event.TryGetMethodDetails() is var (_, methodId))
    {
      var fqn = sharedData.FindMethodFqn(methodId);
      if (context.CommandContext.TargetMethodsRegex is null ||
          context.CommandContext.TargetMethodsRegex.IsMatch(fqn))
      {
        myGrouper.ProcessMethodStartEndEvent(fqn, fqn, context.Event.GetMethodEventKind() == MethodKind.Begin, threadId);
      }

      return;
    }

    if (context.Event is TaskWaitStopArgs or TaskWaitSendArgs)
    {
      LastSeenTaskEvent taskEvent = context.Event switch
      {
        TaskWaitSendArgs sendArgs => new TaskWaitSendEvent { TaskId = sendArgs.TaskID },
        TaskWaitStopArgs stopArgs => new TaskWaitStopEvent { TaskId = stopArgs.TaskID },
        _ => throw new ArgumentOutOfRangeException()
      };

      myGrouper.ProcessTaskWaitEvent(taskEvent, threadId);
      return;
    }

    myGrouper.ProcessNormalEvent(context.Event.EventName, threadId);
  }

  private static void HandleAsyncMethod(string stateMachineName, List<List<string>> traces)
  {
    Console.WriteLine(stateMachineName);
    foreach (var trace in traces)
    {
      Console.WriteLine("Trace start");
      foreach (var frame in trace)
      {
        Console.WriteLine(frame);
      }

      Console.WriteLine();
    }
  }
}