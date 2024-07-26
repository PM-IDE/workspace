using System.Diagnostics;
using Core.Events.EventRecord;
using Core.Utils;

namespace Core.Methods;

public class OnlineAsyncMethodsGrouper(string asyncMethodsPrefix, Action<string, List<List<EventRecordWithMetadata>>> callback)
{
  private abstract class LastSeenTaskEvent
  {
    public required int TaskId { get; init; }
  }

  private sealed class TaskWaitSendEvent : LastSeenTaskEvent;

  private sealed class TaskWaitStopEvent : LastSeenTaskEvent;

  private record AsyncMethodTrace(LastSeenTaskEvent? BeforeTaskEvent, IList<EventRecordWithMetadata> Events)
  {
    public LastSeenTaskEvent? AfterTaskEvent { get; set; }
  }

  private class ThreadData
  {
    public Stack<AsyncMethodTrace> LastTraceStack { get; } = new();
    public LastSeenTaskEvent? LastSeenTaskEvent { get; set; }
  }


  private const string MoveNextMethod = "MoveNext";
  private const string MoveNextWithDot = $".{MoveNextMethod}";

  private readonly Dictionary<string, List<AsyncMethodTrace>> myAsyncMethodsToTraces = new();
  private readonly Dictionary<long, ThreadData> myThreadsData = new();
  private readonly Dictionary<string, string> myAsyncMethodsToTypeNames = new();
  private readonly Dictionary<int, AsyncMethodTrace> myTasksToTracesIds = new();
  private readonly Dictionary<AsyncMethodTrace, int> myTracesToTasksIds = new();


  public void ProcessEvent(EventRecordWithMetadata @event)
  {
    UpdateAsyncMethodsToTypeNames(@event);
    ProcessEventInternal(@event);
  }

  private void ProcessEventInternal(EventRecordWithMetadata eventRecord)
  {
    var threadData = GetThreadData(eventRecord);

    if (TryProcessTaskEvent(eventRecord, threadData)) return;
    if (TryProcessMethodEvent(eventRecord, threadData)) return;

    AppendEventToTraceIfHaveSome(eventRecord);
  }

  private bool TryProcessTaskEvent(EventRecordWithMetadata eventRecord, ThreadData threadData)
  {
    if (!eventRecord.IsTaskWaitSendOrStopEvent()) return false;

    threadData.LastSeenTaskEvent = ToLastSeenTaskEvent(eventRecord);
    AppendEventToTraceIfHaveSome(eventRecord);
    return true;
  }

  private bool TryProcessMethodEvent(EventRecordWithMetadata eventRecord, ThreadData threadData)
  {
    if (eventRecord.TryGetMethodStartEndEventInfo() is not var (frame, isStart) ||
        !myAsyncMethodsToTypeNames.TryGetValue(frame, out var frameName))
    {
      return false;
    }

    var stateMachineName = $"{asyncMethodsPrefix}{frameName}";

    if (isStart)
    {
      ProcessMethodStart(eventRecord, threadData, stateMachineName);
    }
    else
    {
      ProcessMethodEnd(eventRecord, threadData, stateMachineName);
    }

    threadData.LastSeenTaskEvent = null;
    return true;

  }

  private void ProcessMethodStart(EventRecordWithMetadata eventRecord, ThreadData threadData, string stateMachineName)
  {
    var listOfEvents = new List<EventRecordWithMetadata> { eventRecord };
    var newTrace = new AsyncMethodTrace(threadData.LastSeenTaskEvent, listOfEvents);
    if (newTrace.BeforeTaskEvent is TaskWaitStopEvent { TaskId: var waitedTaskId })
    {
      Debug.Assert(!myTasksToTracesIds.ContainsKey(waitedTaskId));
      myTasksToTracesIds[waitedTaskId] = newTrace;
    }

    var listOfAsyncTraces = myAsyncMethodsToTraces.GetOrCreate(stateMachineName, () => []);

    listOfAsyncTraces.Add(newTrace);
    threadData.LastTraceStack.Push(newTrace);
  }

  private void ProcessMethodEnd(EventRecordWithMetadata eventRecord, ThreadData threadData, string stateMachineName)
  {
    Debug.Assert(threadData.LastTraceStack.Count > 0);
    var lastTrace = threadData.LastTraceStack.Pop();
    lastTrace.Events.Add(eventRecord);
    if (threadData.LastSeenTaskEvent is { } lastSeenTaskEvent)
    {
      lastTrace.AfterTaskEvent = lastSeenTaskEvent;
    }

    if (lastTrace.AfterTaskEvent is TaskWaitSendEvent { TaskId: var scheduledTaskId } )
    {
      Debug.Assert(!myTracesToTasksIds.ContainsKey(lastTrace));
      myTracesToTasksIds[lastTrace] = scheduledTaskId;
    }

    DiscoverLogicalExecutions(stateMachineName);
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

  private void DiscoverLogicalExecutions(string stateMachineName)
  {
    var asyncMethods = DiscoverLogicalExecutions(myAsyncMethodsToTraces[stateMachineName]);
    var trace = asyncMethods.Select(traces => traces.SelectMany(t => t.Events).ToList()).ToList();
    if (trace.Count == 0) return;

    foreach (var usedTrace in asyncMethods.SelectMany(m => m))
    {
      myAsyncMethodsToTraces[stateMachineName].Remove(usedTrace);
    }

    callback(stateMachineName, trace);
  }

  private List<List<AsyncMethodTrace>> DiscoverLogicalExecutions(IReadOnlyList<AsyncMethodTrace> traces)
  {
    var result = new List<List<AsyncMethodTrace>>();
    foreach (var startingPoint in FindEntryPoints(traces))
    {
      var logicalExecution = new List<AsyncMethodTrace>();
      var currentTrace = startingPoint;

      var finishedExecution = true;
      while (true)
      {
        logicalExecution.Add(currentTrace);

        if (!myTracesToTasksIds.TryGetValue(currentTrace, out var queuedTaskId)) break;

        if (!myTasksToTracesIds.TryGetValue(queuedTaskId, out currentTrace))
        {
          finishedExecution = false;
          break;
        }
      }

      if (finishedExecution)
      {
        result.Add(logicalExecution);
      }
    }

    return result;
  }


  private IEnumerable<AsyncMethodTrace> FindEntryPoints(IEnumerable<AsyncMethodTrace> traces)
  {
    return traces.Where(IsTraceAnEntryPoint).ToHashSet();
  }

  private bool IsTraceAnEntryPoint(AsyncMethodTrace trace) =>
    trace.BeforeTaskEvent is null ||
    trace.BeforeTaskEvent is not TaskWaitStopEvent { TaskId: var id } ||
    !myTasksToTracesIds.ContainsKey(id);

  private void UpdateAsyncMethodsToTypeNames(EventRecordWithMetadata @event)
  {
    if (@event.TryGetMethodStartEndEventInfo() is not { Frame: var fullMethodName }) return;

    var fullNameWithoutSignature = fullMethodName.AsSpan();
    fullNameWithoutSignature = fullNameWithoutSignature[..fullMethodName.IndexOf('[')];

    if (!fullNameWithoutSignature.Contains('+')) return;
    if (!fullNameWithoutSignature.EndsWith(MoveNextWithDot)) return;

    var stateMachineEnd = fullNameWithoutSignature.IndexOf(MoveNextWithDot, StringComparison.Ordinal);
    var stateMachineStart = fullNameWithoutSignature.LastIndexOf('+');
    if (stateMachineStart >= stateMachineEnd) return;

    var stateMachineType = fullMethodName.AsSpan(stateMachineStart + 1, stateMachineEnd - (stateMachineStart + 1));
    if (!RoslynGeneratedNamesParser.TryParseGeneratedName(stateMachineType, out var kind, out _, out _) ||
        kind != RoslynGeneratedNameKind.StateMachineType)
    {
      return;
    }

    var typeNameStart = fullNameWithoutSignature.IndexOf('!');
    if (typeNameStart < 0) typeNameStart = 0;

    myAsyncMethodsToTypeNames[fullMethodName] = fullMethodName.Substring(typeNameStart, stateMachineEnd - typeNameStart);
  }

  private void AppendEventToTraceIfHaveSome(EventRecordWithMetadata @event)
  {
    if (GetThreadData(@event).LastTraceStack.TryPeek(out var topTrace) && topTrace is { Events: { } eventsList })
    {
      eventsList.Add(@event.DeepClone());
    }
  }

  private ThreadData GetThreadData(EventRecordWithMetadata @event)
  {
    return myThreadsData.GetOrCreate(@event.ManagedThreadId, static () => new ThreadData());
  }
}