using Core.Container;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core.Handlers;

public class CompletedMethodExecutionEvent : IEventPipeStreamEvent
{
  public required TargetMethodFrame Frame { get; init; }
}

[AppComponent]
public class CompletedMethodExecutionHandler : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (eventPipeStreamEvent is not CompletedMethodExecutionEvent @event) return;

    foreach (var frame in @event.Frame.InnerEvents)
    {
      Console.WriteLine(frame.EventName);
    }

    Console.WriteLine();
  }
}