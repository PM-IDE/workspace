using Core.Container;
using Core.Events.EventRecord;
using Core.Methods;
using Core.Utils;
using ProcfilerOnline.Core.Handlers;

namespace ProcfilerOnline.Core.Processors;

[AppComponent]
public class AsyncMethodsProcessor : ITraceEventProcessor
{
  private readonly ICompositeEventPipeStreamEventHandler myHandler;
  private readonly OnlineAsyncMethodsGrouper<EventRecordWithMetadata> myGrouper;

  private string? myApplicationName;


  public AsyncMethodsProcessor(IProcfilerLogger logger, ICompositeEventPipeStreamEventHandler handler)
  {
    myHandler = handler;
    myGrouper = new OnlineAsyncMethodsGrouper<EventRecordWithMetadata>(logger, "ASYNC_", HandleAsyncMethod);
  }


  public void Process(EventProcessingContext context)
  {
    myApplicationName = context.CommandContext.ApplicationName;
    var threadId = context.Event.ManagedThreadId;

    if (context.Event.TryGetMethodDetails() is var (_, methodId))
    {
      if (context.SharedData.FindMethodName(methodId) is not { } fqn) return;

      if (context.CommandContext.TargetMethodsRegex is null ||
          context.CommandContext.TargetMethodsRegex.IsMatch(fqn))
      {
        myGrouper.ProcessMethodStartEndEvent(context.Event, fqn, context.Event.GetMethodEventKind() == MethodKind.Begin, threadId);
      }
    }
    else if (context.Event.ToTaskEvent() is { } taskEvent)
    {
      myGrouper.ProcessTaskEvent(taskEvent, threadId);
    }
    else
    {
      myGrouper.ProcessNormalEvent(context.Event, threadId);
    }
  }

  private void HandleAsyncMethod(string stateMachineName, List<List<EventRecordWithMetadata>> traces)
  {
    if (traces.Count == 0) return;
    var methodInfo = traces.First().First().TryGetExtendedMethodInfo()?.ExtendedMethodInfo;

    myHandler.Handle(new CompletedAsyncMethodEvent
    {
      ApplicationName = myApplicationName ?? "UNDEF_APPLICATION",
      MethodTraces = traces,
      StateMachineName = stateMachineName,
      MethodInfo = methodInfo
    });
  }
}