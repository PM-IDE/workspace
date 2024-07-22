using Core.Container;

namespace ProcfilerOnline.Core.Handlers;

public interface IEventPipeStreamEventHandler
{
  void Handle(IEventPipeStreamEvent eventPipeStreamEvent);
}

public interface IEventPipeStreamEvent;

public interface ICompositeEventPipeStreamEventHandler
{
  void Handle(IEventPipeStreamEvent eventPipeStreamEvent);
}

[AppComponent]
public class CompositeEventPipeStreamEventHandler(IEnumerable<IEventPipeStreamEventHandler> handlers)
  : ICompositeEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent @event)
  {
    foreach (var handler in handlers)
    {
      handler.Handle(@event);
    }
  }
}