using System.Text.RegularExpressions;
using Core.Collector;
using Core.Container;
using Core.Utils;
using Microsoft.Extensions.Logging;
using Procfiler.Core.EventRecord.EventRecord;
using ProcfilerOnline.Core.Handlers;

namespace ProcfilerOnline.Core.Processors;

public readonly record struct MethodFrame(bool IsStart, long MethodId, long QpcStamp);

public class TargetMethodFrame(long methodId)
{
  public long MethodId { get; } = methodId;
  public List<MethodFrame> InnerFrames { get; } = [];
}

[AppComponent]
public class SingleThreadMethodsProcessor(
  IProcfilerLogger logger,
  ICompositeEventPipeStreamEventHandler handler,
  ISharedEventPipeStreamData sharedData
) : ITraceEventProcessor
{
  private readonly Dictionary<long, Stack<TargetMethodFrame>> myStacksPerThreads = new();


  public void Process(EventProcessingContext context)
  {
    if (context.Event.TryGetMethodDetails() is not var (qpcStamp, methodId)) return;

    var eventRecord = context.Event;
    var threadId = eventRecord.ManagedThreadId;
    var isTargetMethod = IsTargetMethod(methodId, context.CommandContext.TargetMethodsRegex);
    var threadStack = myStacksPerThreads.GetOrCreate(threadId, static () => new Stack<TargetMethodFrame>());

    switch (eventRecord.GetMethodEventKind())
    {
      case MethodKind.Begin:
      {
        if (isTargetMethod)
        {
          threadStack.Push(new TargetMethodFrame(methodId));
        }

        foreach (var targetFrame in threadStack)
        {
          targetFrame.InnerFrames.Add(new MethodFrame(true, methodId, qpcStamp));
        }

        break;
      }
      case MethodKind.End:
      {
        foreach (var targetFrame in threadStack)
        {
          targetFrame.InnerFrames.Add(new MethodFrame(false, methodId, qpcStamp));
        }

        if (isTargetMethod)
        {
          if (methodId != threadStack.Peek().MethodId)
          {
            logger.LogWarning("The stack is corrupt for thread {ThreadId}", threadId);
          }

          var frame = threadStack.Pop();

          if (sharedData.FindMethodFqn(frame.MethodId) is not { } methodFqn) return;

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

  private bool IsTargetMethod(long methodId, Regex? targetMethodsRegex)
  {
    if (sharedData.FindMethodFqn(methodId) is not { } methodFqn) return false;

    return targetMethodsRegex is null || targetMethodsRegex.IsMatch(methodFqn);
  }
}