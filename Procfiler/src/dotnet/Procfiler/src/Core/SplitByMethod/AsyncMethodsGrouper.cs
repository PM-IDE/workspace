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
    IDictionary<long, IEventsCollection> managedThreadsEvents);
}

[AppComponent]
public class AsyncMethodsGrouper(IProcfilerLogger logger) : IAsyncMethodsGrouper
{
  public string AsyncMethodsPrefix => "ASYNC_";


  public IDictionary<string, List<List<EventRecordWithMetadata>>> GroupAsyncMethods(
    IDictionary<long, IEventsCollection> managedThreadsEvents)
  {
    var result = new Dictionary<string, List<List<EventRecordWithMetadata>>>();
    var onlineGrouper = new OnlineAsyncMethodsGrouper<EventRecordWithMetadata>(logger, AsyncMethodsPrefix, (method, traces) =>
    {
      result.GetOrCreate(method, static () => []).AddRange(traces);
    });

    foreach (var (_, events) in managedThreadsEvents)
    {
      foreach (var (_, eventRecord) in events)
      {
        var threadId = eventRecord.ManagedThreadId;
        if (eventRecord.IsTaskRelatedEvent())
        {
          if (eventRecord.ToTaskEvent() is { } taskEvent)
          {
            if (taskEvent is TaskWaitEvent taskWaitEvent)
            {
              onlineGrouper.ProcessTaskWaitEvent(taskWaitEvent, threadId);
            }
          }
        }
        else if (eventRecord.TryGetMethodStartEndEventInfo() is { IsStart: var isStart, Frame: var frame })
        {
          onlineGrouper.ProcessMethodStartEndEvent(eventRecord, frame, isStart, threadId);
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