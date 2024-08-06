using Core.Container;
using Core.Events.EventRecord;
using Core.Methods;
using Core.Utils;
using ProcfilerTests.Core;

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

  private static void HandleAsyncMethod(string stateMachineName, List<List<EventRecordWithMetadata>> traces)
  {
    Console.WriteLine(stateMachineName);
    foreach (var trace in traces)
    {
      Console.WriteLine("Trace start");
      Console.WriteLine(ProgramMethodCallTreeDumper.CreateDump(trace, null, e => e.TryGetMethodDetails() switch
      {
        { } => (e.EventName, e.GetMethodEventKind() == MethodKind.Begin),
        _ => null
      }));

      Console.WriteLine();
    }
  }
}