using Core.Container;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core.Handlers;

public class CompletedMethodExecutionEvent : IEventPipeStreamEvent
{
  public required TargetMethodFrame Frame { get; init; }
}

[AppComponent]
public class CompletedMethodExecutionHandler(ISharedEventPipeStreamData sharedData) : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (eventPipeStreamEvent is not CompletedMethodExecutionEvent completedMethodExecutionEvent) return;

    foreach (var frame in completedMethodExecutionEvent.Frame.InnerFrames)
    {
      Console.WriteLine(sharedData.FindMethodFqn(frame.MethodId) ?? "???");
    }
  }
}