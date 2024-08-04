using System.Diagnostics;
using Core.Utils;
using Microsoft.Extensions.Logging;

namespace Core.Methods;

public abstract class LastSeenTaskEvent
{
  public required int TaskId { get; init; }
  public required int OriginatingTaskId { get; init; }

  public override string ToString() => $"{GetType().Name} TaskId: {TaskId}, OriginatingTaskId: {OriginatingTaskId}";
}

public sealed class TaskWaitSendEvent : LastSeenTaskEvent
{
  public required int ContinueWithTaskId { get; init; }
  public required bool IsAsync { get; init; }

  public override string ToString() =>
    $"{base.ToString()}, {nameof(ContinueWithTaskId)}: {ContinueWithTaskId}, {nameof(IsAsync)}: {IsAsync}";
}

public sealed class TaskWaitStopEvent : LastSeenTaskEvent;

public class OnlineAsyncMethodsGrouper<TEvent>(
  IProcfilerLogger logger, string asyncMethodsPrefix, Action<string, List<List<TEvent>>> callback)
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

  private class AsyncMethodTrace(LastSeenTaskEvent? beforeTaskEvent, IList<AsyncMethodEvent> events)
  {
    public LastSeenTaskEvent? BeforeTaskEvent { get; } = beforeTaskEvent;
    public IList<AsyncMethodEvent> Events { get; } = events;

    public bool Completed { get; set; }
    public LastSeenTaskEvent? AfterTaskEvent { get; set; }
  }

  private class ThreadData
  {
    public Stack<AsyncMethodTrace> AsyncMethodsStack { get; } = new();
    public LastSeenTaskEvent? LastSeenTaskEvent { get; set; }
  }


  private const string MoveNextMethod = "MoveNext";
  private const string MoveNextWithDot = $".{MoveNextMethod}";

  private readonly Dictionary<string, List<AsyncMethodTrace>> myAsyncMethodsToTraces = new();
  private readonly Dictionary<long, ThreadData> myThreadsData = new();
  private readonly Dictionary<string, string> myAsyncMethodsToTypeNames = new();
  private readonly Dictionary<int, AsyncMethodTrace> myTasksToTracesIds = new();
  private readonly Dictionary<AsyncMethodTrace, int> myTracesToTasksIds = new();


  public void ProcessTaskWaitEvent(LastSeenTaskEvent taskEvent, long managedThreadId)
  {
    logger.LogDebug("[{ThreadId}]: {TaskEvent}", managedThreadId, taskEvent);
    GetThreadData(managedThreadId).LastSeenTaskEvent = taskEvent;
  }

  public void ProcessMethodStartEndEvent(TEvent @event, string fullMethodName, bool isStart, long managedThreadId)
  {
    UpdateAsyncMethodsToTypeNames(fullMethodName);
    if (!myAsyncMethodsToTypeNames.TryGetValue(fullMethodName, out var frameName))
    {
      return;
    }

    logger.LogDebug("[{ThreadId}]: Method[{Start}]: {Fqn}", managedThreadId, isStart, fullMethodName);

    var stateMachineName = $"{asyncMethodsPrefix}{frameName}";
    var threadData = GetThreadData(managedThreadId);

    if (isStart)
    {
      ProcessMethodStart(@event, threadData, stateMachineName);
    }
    else
    {
      ProcessMethodEnd(@event, threadData, stateMachineName);
    }

    threadData.LastSeenTaskEvent = null;
  }

  public void ProcessNormalEvent(TEvent @event, long managedThreadId)
  {
    AppendEventToTraceIfHaveSome(managedThreadId, @event);
  }

  private void ProcessMethodStart(TEvent eventRecord, ThreadData threadData, string stateMachineName)
  {
    var listOfEvents = new List<AsyncMethodEvent> { new DefaultEvent(eventRecord) };
    var newTrace = new AsyncMethodTrace(threadData.LastSeenTaskEvent, listOfEvents);

    if (newTrace.BeforeTaskEvent is TaskWaitStopEvent { TaskId: var waitedTaskId })
    {
      Debug.Assert(!myTasksToTracesIds.ContainsKey(waitedTaskId));
      myTasksToTracesIds[waitedTaskId] = newTrace;
    }

    var listOfAsyncTraces = myAsyncMethodsToTraces.GetOrCreate(stateMachineName, () => []);

    listOfAsyncTraces.Add(newTrace);
    threadData.AsyncMethodsStack.Push(newTrace);
  }

  private void ProcessMethodEnd(TEvent eventRecord, ThreadData threadData, string stateMachineName)
  {
    Debug.Assert(threadData.AsyncMethodsStack.Count > 0);

    var lastTrace = threadData.AsyncMethodsStack.Pop();
    lastTrace.Events.Add(new DefaultEvent(eventRecord));

    if (threadData.LastSeenTaskEvent is { } lastSeenTaskEvent)
    {
      lastTrace.AfterTaskEvent = lastSeenTaskEvent;
    }

    if (lastTrace.AfterTaskEvent is TaskWaitSendEvent { TaskId: var scheduledTaskId } )
    {
      Debug.Assert(!myTracesToTasksIds.ContainsKey(lastTrace));
      myTracesToTasksIds[lastTrace] = scheduledTaskId;
    }

    lastTrace.Completed = true;

    if (IsTraceAnEntryPoint(lastTrace))
    {
      foreach (var asyncMethod in threadData.AsyncMethodsStack)
      {
        if (!asyncMethod.Completed)
        {
          asyncMethod.Events.Add(new InnerAsyncMethodEvent(lastTrace));
        }
      }
    }

    DiscoverLogicalExecutions(stateMachineName);
  }

  private void DiscoverLogicalExecutions(string stateMachineName)
  {
    var asyncMethods = DiscoverLogicalExecutions(myAsyncMethodsToTraces[stateMachineName]);
    if (asyncMethods.Count == 0) return;

    foreach (var usedTrace in asyncMethods.SelectMany(m => m))
    {
      myAsyncMethodsToTraces[stateMachineName].Remove(usedTrace);
    }

    callback(stateMachineName, MaterializeDefaultEventTraces(asyncMethods));
  }

  private List<List<TEvent>> MaterializeDefaultEventTraces(List<List<AsyncMethodTrace>> traces)
  {
    var result = new List<List<TEvent>>();

    foreach (var trace in traces)
    {
      var newTrace = new List<TEvent>();
      MaterializeTrace(newTrace, trace);

      result.Add(newTrace);
    }

    return result;
  }

  private void MaterializeTrace(List<TEvent> result, List<AsyncMethodTrace> logicalExecution)
  {
    foreach (var trace in logicalExecution)
    {
      foreach (var @event in trace.Events)
      {
        switch (@event)
        {
          case DefaultEvent { Event: var defaultEvent }:
            result.Add(defaultEvent);
            break;
          case InnerAsyncMethodEvent innerAsyncMethodEvent:
            var nestedFirstTrace = innerAsyncMethodEvent.NestedAsyncMethodStart;
            if (nestedFirstTrace.AfterTaskEvent is TaskWaitSendEvent { ContinueWithTaskId: var continueWithTaskId } &&
                trace.AfterTaskEvent is TaskWaitSendEvent { TaskId: var taskId } &&
                continueWithTaskId == taskId &&
                DiscoverLogicalExecution(innerAsyncMethodEvent.NestedAsyncMethodStart) is { } innerLogicalExecution)
            {
              MaterializeTrace(result, innerLogicalExecution);
            }

            break;
        }
      }
    }
  }

  private List<List<AsyncMethodTrace>> DiscoverLogicalExecutions(IReadOnlyList<AsyncMethodTrace> traces)
  {
    var result = new List<List<AsyncMethodTrace>>();
    foreach (var startingPoint in FindEntryPoints(traces))
    {
      if (DiscoverLogicalExecution(startingPoint) is { } logicalExecution)
      {
        result.Add(logicalExecution);
      }
    }

    return result;
  }

  private List<AsyncMethodTrace>? DiscoverLogicalExecution(AsyncMethodTrace startingPoint)
  {
    var logicalExecution = new List<AsyncMethodTrace>();
    var currentTrace = startingPoint;

    var finishedExecution = true;
    while (true)
    {
      if (!currentTrace.Completed)
      {
        finishedExecution = false;
        break;
      }

      logicalExecution.Add(currentTrace);

      if (!myTracesToTasksIds.TryGetValue(currentTrace, out var queuedTaskId)) break;

      if (!myTasksToTracesIds.TryGetValue(queuedTaskId, out currentTrace))
      {
        finishedExecution = false;
        break;
      }
    }

    return finishedExecution ? logicalExecution : null;
  }

  private IEnumerable<AsyncMethodTrace> FindEntryPoints(IEnumerable<AsyncMethodTrace> traces) =>
    traces.Where(IsTraceAnEntryPoint).ToHashSet();

  private bool IsTraceAnEntryPoint(AsyncMethodTrace trace) =>
    trace.Completed && (
      trace.BeforeTaskEvent is not TaskWaitStopEvent { TaskId: var id } ||
      !myTasksToTracesIds.ContainsKey(id)
    );

  private void UpdateAsyncMethodsToTypeNames(string fullMethodName)
  {
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

  private void AppendEventToTraceIfHaveSome(long managedThreadId, TEvent @event)
  {
    if (GetThreadData(managedThreadId).AsyncMethodsStack.TryPeek(out var topTrace) && topTrace is { Events: { } eventsList })
    {
      eventsList.Add(new DefaultEvent(@event));
    }
  }

  private ThreadData GetThreadData(long managedThreadId)
  {
    return myThreadsData.GetOrCreate(managedThreadId, static () => new ThreadData());
  }
}