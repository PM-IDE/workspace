using Core.Container;
using Core.Utils;
using Microsoft.Extensions.Logging;

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
public class CompositeEventPipeStreamEventHandler(IProcfilerLogger logger, IEnumerable<IEventPipeStreamEventHandler> handlers)
  : ICompositeEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent @event)
  {
    Task.Run(() =>
    {
      foreach (var handler in handlers)
      {
        try
        {
          handler.Handle(@event);
        }
        catch (Exception ex)
        {
          logger.LogError(ex, "Failed to execute handler {Handler}", handler.GetType().Name);
        }
      }
    });
  }
}