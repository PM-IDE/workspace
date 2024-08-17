using System.Text.RegularExpressions;
using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Core.Handlers;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core;

public interface IThreadsMethodsProcessor
{
  void Process(EventProcessingContext context);
  IReadOnlyList<(long ThreadId, List<EventRecordWithMetadata>)> ReclaimNotClosedMethods();
}

public class TargetMethodFrame(long methodId)
{
  public long MethodId { get; } = methodId;
  public List<EventRecordWithMetadata> InnerEvents { get; } = [];
}

[AppComponent]
public class ThreadsMethodsProcessor(
  IProcfilerLogger logger,
  ICompositeEventPipeStreamEventHandler handler,
  IEventProcessingEntryPoint eventProcessingEntryPoint
) : IThreadsMethodsProcessor
{
  private readonly Dictionary<long, Stack<TargetMethodFrame>> myStacksPerThreads = new();


  public void Process(EventProcessingContext context)
  {
    if (context.Event.IsExceptionCatcherEnter(out var functionId))
    {
      var threadId = context.Event.ManagedThreadId;
      var threadStack = myStacksPerThreads.GetOrCreate(threadId, static () => new Stack<TargetMethodFrame>());
      while (threadStack.Count > 0 && threadStack.Peek().MethodId != functionId)
      {
        ProcessInternal(context with
        {
          Event = threadStack.Peek().InnerEvents.First().ConvertToMethodEndEvent()
        });
      }

      return;
    }

    ProcessInternal(context);
  }

  private void ProcessInternal(EventProcessingContext context)
  {
    var eventRecord = context.Event;
    var threadId = eventRecord.ManagedThreadId;
    var threadStack = myStacksPerThreads.GetOrCreate(threadId, static () => new Stack<TargetMethodFrame>());

    foreach (var targetFrame in threadStack)
    {
      targetFrame.InnerEvents.Add(eventRecord);
    }

    if (eventRecord.TryGetMethodDetails() is var (_, methodId))
    {
      var isTargetMethod = IsTargetMethod(context, methodId, context.CommandContext.TargetMethodsRegex);

      switch (eventRecord.GetMethodEventKind())
      {
        case MethodKind.Begin:
        {
          if (isTargetMethod)
          {
            threadStack.Push(new TargetMethodFrame(methodId));
            threadStack.Peek().InnerEvents.Add(eventRecord);
          }

          break;
        }
        case MethodKind.End:
        {
          if (isTargetMethod)
          {
            if (methodId != threadStack.Peek().MethodId)
            {
              logger.LogWarning("The stack is corrupt for thread {ThreadId}", threadId);
            }

            var frame = threadStack.Pop();

            if (context.SharedData.FindMethodName(methodId) is not { } methodFqn) return;

            if (context.CommandContext.TargetMethodsRegex is null ||
                context.CommandContext.TargetMethodsRegex.IsMatch(methodFqn))
            {
              handler.Handle(new CompletedMethodExecutionEvent
              {
                Frame = frame
              });
            }
          }

          break;
        }
        default:
          throw new ArgumentOutOfRangeException();
      }
    }

    eventProcessingEntryPoint.Process(context);
  }

  public IReadOnlyList<(long ThreadId, List<EventRecordWithMetadata>)> ReclaimNotClosedMethods()
  {
    return myStacksPerThreads
      .Where(pair => pair.Value.Count > 0)
      .Select(pair => (pair.Key, pair.Value.Select(targetFrame => targetFrame.InnerEvents.First()).ToList()))
      .ToList();
  }

  private bool IsTargetMethod(EventProcessingContext context, long methodId, Regex? targetMethodsRegex)
  {
    if (context.SharedData.FindMethodName(methodId) is not { } methodFqn) return false;

    return targetMethodsRegex is null || targetMethodsRegex.IsMatch(methodFqn);
  }
}