namespace Core.Methods;

public partial class OnlineAsyncMethodsGrouper<TEvent>
{
  private class QueuedAsyncMethodsStorage
  {
    private readonly HashSet<Guid> myRequiredToCacheTraces = [];
    private readonly Dictionary<Guid, List<TEvent>> myCachedTraces = new();
    private readonly Queue<(string StateMachineName, List<AsyncMethodTrace> MethodTraces)> myQueuedAsyncMethods = [];

    public void AddTraceCacheRequest(Guid traceId)
    {
      myRequiredToCacheTraces.Add(traceId);
    }

    public void CacheIfNeeded(Guid traceId, List<TEvent> trace)
    {
      if (myRequiredToCacheTraces.Remove(traceId))
      {
        myCachedTraces[traceId] = trace;
      }
    }

    public void ExecuteWithQueuedAsyncMethods(Action<(string StateMachineName, List<AsyncMethodTrace> MethodTrace)> action)
    {
      var count = myQueuedAsyncMethods.Count;
      for (var i = 0; i < count; ++i)
      {
        action(myQueuedAsyncMethods.Dequeue());
      }
    }

    public void QueueAsyncMethod(string stateMachineName, List<AsyncMethodTrace> methodTraces)
    {
      myQueuedAsyncMethods.Enqueue((stateMachineName, methodTraces));
    }

    public List<TEvent>? DevastateCache(Guid traceId) => myCachedTraces.Remove(traceId, out var trace) ? trace : null;
  }
}