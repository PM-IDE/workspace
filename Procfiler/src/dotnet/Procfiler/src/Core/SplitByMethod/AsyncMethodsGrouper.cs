using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsCollection;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.SplitByMethod;

public interface IAsyncMethodsGrouper
{
  string AsyncMethodsPrefix { get; }


  IDictionary<string, IList<IReadOnlyList<EventRecordWithMetadata>>> GroupAsyncMethods(
    IEnumerable<string> methodsNames,
    IDictionary<long, IEventsCollection> managedThreadsEvents);
}

[AppComponent]
public class AsyncMethodsGrouper(IProcfilerLogger logger) : IAsyncMethodsGrouper
{
  private record AsyncMethodTrace(EventRecordWithMetadata? BeforeTaskEvent, IList<EventRecordWithMetadata> Events)
  {
    public EventRecordWithMetadata? AfterTaskEvent { get; set; }
  }

  private const string MoveNextMethod = "MoveNext";
  private const string MoveNextWithDot = $".{MoveNextMethod}";


  public string AsyncMethodsPrefix => "ASYNC_";


  public IDictionary<string, IList<IReadOnlyList<EventRecordWithMetadata>>> GroupAsyncMethods(
    IEnumerable<string> methodsNames,
    IDictionary<long, IEventsCollection> managedThreadsEvents)
  {
    var asyncMethodsWithTypeNames = FindAllAsyncMoveNextMethods(methodsNames);
    var asyncMethodsToTraces = CreateAsyncMethodsToTracesMap(asyncMethodsWithTypeNames, managedThreadsEvents);

    return DiscoverLogicalAsyncMethodsExecutions(asyncMethodsToTraces);
  }

  private IDictionary<string, List<AsyncMethodTrace>> CreateAsyncMethodsToTracesMap(
    IDictionary<string, string> asyncMethodsWithTypeNames,
    IDictionary<long, IEventsCollection> managedThreadsEvents)
  {
    var asyncMethods = asyncMethodsWithTypeNames.Keys.ToHashSet();
    var asyncMethodsToTraces = new Dictionary<string, List<AsyncMethodTrace>>();

    foreach (var (_, events) in managedThreadsEvents)
    {
      EventRecordWithMetadata? lastSeenTaskEvent = null;
      var lastTracesStack = new Stack<AsyncMethodTrace>();

      void AppendEventToTraceIfHasSome(EventRecordWithMetadata eventRecord)
      {
        if (lastTracesStack.TryPeek(out var topTrace) && topTrace is { Events: { } eventsList })
        {
          eventsList.Add(eventRecord);
        }
      }

      foreach (var (_, eventRecord) in events)
      {
        if (eventRecord.IsTaskWaitSendOrStopEvent())
        {
          lastSeenTaskEvent = eventRecord;
          AppendEventToTraceIfHasSome(eventRecord);
          continue;
        }

        if (eventRecord.TryGetMethodStartEndEventInfo() is var (frame, isStart) &&
            asyncMethods.Contains(frame))
        {
          if (isStart)
          {
            var listOfEvents = new List<EventRecordWithMetadata> { eventRecord };
            var newAsyncMethodTraces = new AsyncMethodTrace(lastSeenTaskEvent, listOfEvents);
            var stateMachineName = $"{AsyncMethodsPrefix}{asyncMethodsWithTypeNames[frame]}";
            var listOfAsyncTraces = asyncMethodsToTraces.GetOrCreate(stateMachineName, () => new List<AsyncMethodTrace>());

            listOfAsyncTraces.Add(newAsyncMethodTraces);
            lastTracesStack.Push(newAsyncMethodTraces);
            lastSeenTaskEvent = null;
          }
          else
          {
            Debug.Assert(lastTracesStack.Count > 0);
            var lastTrace = lastTracesStack.Pop();
            lastTrace.Events.Add(eventRecord);
            if (lastSeenTaskEvent is { })
            {
              lastTrace.AfterTaskEvent = lastSeenTaskEvent;
            }
          }

          continue;
        }

        AppendEventToTraceIfHasSome(eventRecord);
      }

      Debug.Assert(lastTracesStack.Count == 0);
    }

    return asyncMethodsToTraces;
  }

  private Dictionary<string, IList<IReadOnlyList<EventRecordWithMetadata>>> DiscoverLogicalAsyncMethodsExecutions(
    IDictionary<string, List<AsyncMethodTrace>> asyncMethodsTraces)
  {
    return asyncMethodsTraces.ToDictionary(
      pair => pair.Key,
      pair => DiscoverLogicalAsyncExecutions(pair.Value)
    );
  }

  private static IList<IReadOnlyList<EventRecordWithMetadata>> DiscoverLogicalAsyncExecutions(
    IReadOnlyCollection<AsyncMethodTrace> traces)
  {
    var tracesTaskInfos = ExtractTasksInfos(traces);
    var result = new List<List<AsyncMethodTrace>>();
    foreach (var startingPoint in FindEntryPoints(traces, tracesTaskInfos))
    {
      var logicalExecution = new List<AsyncMethodTrace>();
      var currentTrace = startingPoint;

      while (true)
      {
        logicalExecution.Add(currentTrace);
        if (!tracesTaskInfos.TracesToQueuedTasks.TryGetValue(currentTrace, out var queuedTaskId)) break;
        if (!tracesTaskInfos.TasksToWaitedTraces.TryGetValue(queuedTaskId, out currentTrace)) break;
      }

      result.Add(logicalExecution);
    }

    return result.Select(
      trace => (IReadOnlyList<EventRecordWithMetadata>)trace.SelectMany(t => t.Events).ToList()).ToList();
  }

  private static IEnumerable<AsyncMethodTrace> FindEntryPoints(
    IEnumerable<AsyncMethodTrace> traces, AsyncTracesTaskInfo tracesTaskInfo)
  {
    return traces.Where(trace => IsTraceAnEntryPoint(trace, tracesTaskInfo)).ToHashSet();
  }

  private static bool IsTraceAnEntryPoint(AsyncMethodTrace trace, AsyncTracesTaskInfo tracesTaskInfo) =>
    trace.BeforeTaskEvent is null ||
    !trace.BeforeTaskEvent.IsTaskWaitStopEvent(out var id) ||
    !tracesTaskInfo.TasksToWaitedTraces.ContainsKey(id);

  private readonly record struct AsyncTracesTaskInfo(
    Dictionary<int, AsyncMethodTrace> TasksToWaitedTraces,
    Dictionary<AsyncMethodTrace, int> TracesToQueuedTasks
  );

  private static AsyncTracesTaskInfo ExtractTasksInfos(
    IEnumerable<AsyncMethodTrace> traces)
  {
    var tasksToTracesWhoWaited = new Dictionary<int, AsyncMethodTrace>();
    var tracesToQueuedTasks = new Dictionary<AsyncMethodTrace, int>();

    foreach (var asyncMethodTrace in traces)
    {
      if (asyncMethodTrace.BeforeTaskEvent?.IsTaskWaitStopEvent(out var waitedTaskId) ?? false)
      {
        Debug.Assert(!tasksToTracesWhoWaited.ContainsKey(waitedTaskId));
        tasksToTracesWhoWaited[waitedTaskId] = asyncMethodTrace;
      }

      if (asyncMethodTrace.AfterTaskEvent?.IsTaskWaitSendEvent(out var scheduledTaskId) ?? false)
      {
        Debug.Assert(!tracesToQueuedTasks.ContainsKey(asyncMethodTrace));
        tracesToQueuedTasks[asyncMethodTrace] = scheduledTaskId;
      }
    }

    return new AsyncTracesTaskInfo(tasksToTracesWhoWaited, tracesToQueuedTasks);
  }

  private static IDictionary<string, string> FindAllAsyncMoveNextMethods(IEnumerable<string> methodNames)
  {
    var asyncMethods = new Dictionary<string, string>();
    foreach (var fullMethodName in methodNames)
    {
      var fullNameWithoutSignature = fullMethodName.AsSpan();
      fullNameWithoutSignature = fullNameWithoutSignature[..fullMethodName.IndexOf('[')];

      if (!fullNameWithoutSignature.Contains('+')) continue;
      if (!fullNameWithoutSignature.EndsWith(MoveNextWithDot)) continue;

      var stateMachineEnd = fullNameWithoutSignature.IndexOf(MoveNextWithDot, StringComparison.Ordinal);
      var stateMachineStart = fullNameWithoutSignature.LastIndexOf('+');
      if (stateMachineStart >= stateMachineEnd) continue;

      var stateMachineType = fullMethodName.AsSpan(stateMachineStart + 1, stateMachineEnd - (stateMachineStart + 1));
      if (!RoslynGeneratedNamesParser.TryParseGeneratedName(stateMachineType, out var kind, out _, out _) ||
          kind != RoslynGeneratedNameKind.StateMachineType)
      {
        continue;
      }

      var typeNameStart = fullNameWithoutSignature.IndexOf('!');
      if (typeNameStart < 0) typeNameStart = 0;

      asyncMethods[fullMethodName] = fullMethodName.Substring(typeNameStart, stateMachineEnd - typeNameStart);
    }

    return asyncMethods;
  }
}