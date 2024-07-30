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
public class AsyncMethodsGrouper : IAsyncMethodsGrouper
{
  public string AsyncMethodsPrefix => "ASYNC_";


  public IDictionary<string, List<List<EventRecordWithMetadata>>> GroupAsyncMethods(
    IDictionary<long, IEventsCollection> managedThreadsEvents)
  {
    var result = new Dictionary<string, List<List<EventRecordWithMetadata>>>();
    var onlineGrouper = new OnlineAsyncMethodsGrouper<EventRecordWithMetadata>(AsyncMethodsPrefix, (method, traces) =>
    {
      result.GetOrCreate(method, static () => []).AddRange(traces);
    });

    foreach (var (_, events) in managedThreadsEvents)
    {
      foreach (var (_, eventRecord) in events)
      {
        var threadId = eventRecord.ManagedThreadId;
        if (eventRecord.IsTaskWaitSendOrStopEvent())
        {
          onlineGrouper.ProcessTaskWaitEvent(ToLastSeenTaskEvent(eventRecord), threadId);
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

  private static LastSeenTaskEvent ToLastSeenTaskEvent(EventRecordWithMetadata eventRecord)
  {
    if (eventRecord.IsTaskWaitSendEvent(out var sentTaskId))
    {
      return new TaskWaitSendEvent
      {
        TaskId = sentTaskId
      };
    }

    if (eventRecord.IsTaskWaitStopEvent(out var waitedTaskId))
    {
      return new TaskWaitStopEvent
      {
        TaskId = waitedTaskId
      };
    }

    throw new ArgumentOutOfRangeException(eventRecord.EventName);
  }
}