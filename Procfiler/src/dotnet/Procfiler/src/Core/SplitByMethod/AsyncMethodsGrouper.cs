using Core.Container;
using Core.Events.EventRecord;
using Core.Methods;
using Core.Utils;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection;

namespace Procfiler.Core.SplitByMethod;

public interface IAsyncMethodsGrouper
{
  string AsyncMethodsPrefix { get; }


  IDictionary<string, List<List<EventRecordWithMetadata>>> GroupAsyncMethods(
    IDictionary<long, IEventsCollection> managedThreadsEvents,
    bool removeFirstMoveNextCalls
  );
}

[AppComponent]
public class AsyncMethodsGrouper(IProcfilerLogger logger) : IAsyncMethodsGrouper
{
  public string AsyncMethodsPrefix => "ASYNC_";


  public IDictionary<string, List<List<EventRecordWithMetadata>>> GroupAsyncMethods(
    IDictionary<long, IEventsCollection> managedThreadsEvents,
    bool removeFirstMoveNextCalls
  )
  {
    var result = new Dictionary<string, List<List<EventRecordWithMetadata>>>();
    var onlineGrouper = new OnlineAsyncMethodsGrouper<EventRecordWithMetadata>(
      logger,
      AsyncMethodsPrefix,
      (method, traces) =>
      {
        if (removeFirstMoveNextCalls)
        {
          foreach (var trace in traces.Where(trace => trace.Count > 2))
          {
            result.GetOrCreate(method, static () => []).Add(trace[1..^1]);
          }

          return;
        }

        result.GetOrCreate(method, static () => []).AddRange(traces);
      }
    );

    foreach (var (_, events) in managedThreadsEvents)
    {
      foreach (var (_, eventRecord) in events)
      {
        var threadId = eventRecord.ManagedThreadId;
        if (eventRecord.ToTaskEvent() is { } taskEvent)
        {
          onlineGrouper.ProcessTaskEvent(taskEvent, threadId);
        }
        else if (eventRecord.TryGetMethodStartEndEventInfo() is { IsStart: var isStart, Frame: var frame })
        {
          onlineGrouper.ProcessMethodStartEndEvent(eventRecord.DeepClone(), frame, isStart, threadId);
        }
        else
        {
          onlineGrouper.ProcessNormalEvent(eventRecord, threadId);
        }
      }
    }

    return result;
  }
}