using Bxes.Utils;
using Core.Container;
using Core.Utils;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsCollection;

namespace Procfiler.Core.SplitByMethod;

public interface IAsyncMethodsGrouper
{
  string AsyncMethodsPrefix { get; }


  IDictionary<string, IList<IReadOnlyList<EventRecordWithMetadata>>> GroupAsyncMethods(
    IDictionary<long, IEventsCollection> managedThreadsEvents);
}

internal record AsyncMethodTrace(EventRecordWithMetadata? BeforeTaskEvent, IList<EventRecordWithMetadata> Events)
{
  public EventRecordWithMetadata? AfterTaskEvent { get; set; }
}

public class OnlineAsyncMethodsGrouper(string asyncMethodsPrefix, Action<string, List<List<EventRecordWithMetadata>>> callback)
{
  private class ThreadData
  {
    public Stack<AsyncMethodTrace> LastTraceStack { get; } = new();
    public EventRecordWithMetadata? LastSeenTaskEvent { get; set; }
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
    if (eventRecord.IsTaskWaitSendOrStopEvent())
    {
      threadData.LastSeenTaskEvent = eventRecord;
      AppendEventToTraceIfHaveSome(eventRecord);
      return;
    }

    if (eventRecord.TryGetMethodStartEndEventInfo() is var (frame, isStart) &&
        myAsyncMethodsToTypeNames.ContainsKey(frame))
    {
      var stateMachineName = $"{asyncMethodsPrefix}{myAsyncMethodsToTypeNames[frame]}";

      if (isStart)
      {
        var listOfEvents = new List<EventRecordWithMetadata> { eventRecord };
        var newTrace = new AsyncMethodTrace(threadData.LastSeenTaskEvent, listOfEvents);
        if (newTrace.BeforeTaskEvent?.IsTaskWaitStopEvent(out var waitedTaskId) ?? false)
        {
          Debug.Assert(!myTasksToTracesIds.ContainsKey(waitedTaskId));
          myTasksToTracesIds[waitedTaskId] = newTrace;
        }

        var listOfAsyncTraces = myAsyncMethodsToTraces.GetOrCreate(stateMachineName, () => []);

        listOfAsyncTraces.Add(newTrace);
        threadData.LastTraceStack.Push(newTrace);
        threadData.LastSeenTaskEvent = null;
      }
      else
      {
        Debug.Assert(threadData.LastTraceStack.Count > 0);
        var lastTrace = threadData.LastTraceStack.Pop();
        lastTrace.Events.Add(eventRecord);
        if (threadData.LastSeenTaskEvent is { } lastSeenTaskEvent)
        {
          lastTrace.AfterTaskEvent = lastSeenTaskEvent;
        }

        if (lastTrace.AfterTaskEvent?.IsTaskWaitSendEvent(out var scheduledTaskId) ?? false)
        {
          Debug.Assert(!myTracesToTasksIds.ContainsKey(lastTrace));
          myTracesToTasksIds[lastTrace] = scheduledTaskId;
        }

        DiscoverLogicalExecutions(stateMachineName);
      }

      return;
    }

    AppendEventToTraceIfHaveSome(eventRecord);
  }

  private void DiscoverLogicalExecutions(string stateMachineName)
  {
    var result = new Dictionary<string, List<List<EventRecordWithMetadata>>>();
    var asyncMethods = DiscoverLogicalExecutions(myAsyncMethodsToTraces[stateMachineName]);
    result[stateMachineName] = asyncMethods.Select(traces => traces.SelectMany(t => t.Events).ToList()).ToList();

    foreach (var usedTrace in asyncMethods.SelectMany(m => m))
    {
      myAsyncMethodsToTraces[stateMachineName].Remove(usedTrace);
    }

    foreach (var (name, trace) in result)
    {
      callback(name, trace);
    }
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
    !trace.BeforeTaskEvent.IsTaskWaitStopEvent(out var id) ||
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
      eventsList.Add(@event);
    }
  }

  private ThreadData GetThreadData(EventRecordWithMetadata @event)
  {
    return myThreadsData.GetOrCreate(@event.ManagedThreadId, static () => new ThreadData());
  }
}

[AppComponent]
public class AsyncMethodsGrouper(IProcfilerLogger logger) : IAsyncMethodsGrouper
{
  public string AsyncMethodsPrefix => "ASYNC_";


  public IDictionary<string, IList<IReadOnlyList<EventRecordWithMetadata>>> GroupAsyncMethods(
    IDictionary<long, IEventsCollection> managedThreadsEvents)
  {
    var result = new Dictionary<string, IList<IReadOnlyList<EventRecordWithMetadata>>>();
    var onlineGrouper = new OnlineAsyncMethodsGrouper(AsyncMethodsPrefix, (method, traces) =>
    {
      result.GetOrCreate(method, static () => []).AddRange(traces);
    });

    foreach (var (_, events) in managedThreadsEvents)
    {
      foreach (var @event in events)
      {
        onlineGrouper.ProcessEvent(@event.Event.DeepClone());
      }
    }

    return result;
  }
}