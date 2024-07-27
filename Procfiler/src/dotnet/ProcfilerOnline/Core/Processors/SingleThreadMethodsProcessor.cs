using System.Text.RegularExpressions;
using Core.Collector;
using Core.Container;
using Core.Utils;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Core.Handlers;

namespace ProcfilerOnline.Core.Processors;

public readonly record struct MethodFrame(bool IsStart, ulong MethodId, long QpcStamp);

public class TargetMethodFrame(ulong methodId)
{
  public ulong MethodId { get; } = methodId;
  public List<MethodFrame> InnerFrames { get; } = [];
}

[AppComponent]
public class SingleThreadMethodsProcessor(
  IProcfilerLogger logger,
  ICompositeEventPipeStreamEventHandler handler,
  ISharedEventPipeStreamData sharedData
) : ITraceEventProcessor
{
  private readonly Dictionary<int, Stack<TargetMethodFrame>> myStacksPerThreads = new();


  public void Process(EventProcessingContext context)
  {
    if (context.Event.ProviderName is not EventPipeProvidersNames.ProcfilerCppProvider) return;

    var traceEvent = context.Event;
    var threadId = traceEvent.ThreadID;
    var qpcStamp = (long)traceEvent.PayloadValue(0);
    var methodId = (ulong)traceEvent.PayloadValue(1);
    var isTargetMethod = IsTargetMethod(methodId, context.CommandContext.TargetMethodsRegex);
    var threadStack = myStacksPerThreads.GetOrCreate(threadId, static () => new Stack<TargetMethodFrame>());

    switch (traceEvent.GetMethodEventKind())
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

          handler.Handle(new CompletedMethodExecutionEvent
          {
            Frame = threadStack.Pop()
          });
        }

        break;
      }
      default:
        throw new ArgumentOutOfRangeException();
    }
  }

  private bool IsTargetMethod(ulong methodId, Regex? targetMethodsRegex)
  {
    if (sharedData.FindMethodFqn(methodId) is not { } methodFqn) return false;

    return targetMethodsRegex is null || targetMethodsRegex.IsMatch(methodFqn);
  }
}