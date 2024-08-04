namespace Core.Methods;

public partial class OnlineAsyncMethodsGrouper<TEvent>
{
  private abstract class AsyncMethodEvent;

  private sealed class DefaultEvent(TEvent @event) : AsyncMethodEvent
  {
    public TEvent Event { get; } = @event;
  }

  private sealed class InnerAsyncMethodEvent(AsyncMethodTrace startTrace) : AsyncMethodEvent
  {
    public AsyncMethodTrace NestedAsyncMethodStart { get; } = startTrace;
  }

  private class AsyncMethodTrace(TaskEvent? beforeTaskEvent, IList<AsyncMethodEvent> events)
  {
    public Guid TraceId { get; } = Guid.NewGuid();
    public TaskEvent? BeforeTaskEvent { get; } = beforeTaskEvent;
    public IList<AsyncMethodEvent> Events { get; } = events;

    public bool Completed { get; set; }
    public TaskEvent? AfterTaskEvent { get; set; }
  }

  private class ThreadData
  {
    public Stack<AsyncMethodTrace> AsyncMethodsStack { get; } = new();
    public TaskEvent? LastSeenTaskEvent { get; set; }
  }
}