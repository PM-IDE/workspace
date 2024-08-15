using Core.Container;
using Core.Events.EventRecord;
using ProcfilerTests.Core;

namespace ProcfilerOnline.Core.Handlers;

public class CompletedAsyncMethodEvent : IEventPipeStreamEvent
{
  public required string StateMachineName { get; init; }
  public required List<List<EventRecordWithMetadata>> MethodTraces { get; init; }
}

[AppComponent]
public class CompletedAsyncMethodHandler : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (eventPipeStreamEvent is not CompletedAsyncMethodEvent completedAsyncMethodEvent) return;

    Console.WriteLine(completedAsyncMethodEvent.StateMachineName);
    foreach (var trace in completedAsyncMethodEvent.MethodTraces)
    {
      Console.WriteLine("Trace start");
      Console.WriteLine(ProgramMethodCallTreeDumper.CreateDump(trace, null, HandlerUtil.ExtractFrame));

      Console.WriteLine();
    }
  }
}