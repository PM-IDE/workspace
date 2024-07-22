using System.Text.RegularExpressions;
using Core.Utils;
using Microsoft.Diagnostics.Tracing;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Core.Handlers;

namespace ProcfilerOnline.Core;

public readonly record struct MethodFrame(bool IsStart, ulong MethodId, long QpcStamp);

public class TargetMethodFrame(ulong methodId)
{
  public ulong MethodId { get; } = methodId;
  public List<MethodFrame> InnerFrames { get; } = [];
}

public class ThreadEventsProcessor(
  IProcfilerLogger logger,
  ICompositeEventPipeStreamEventHandler handler,
  ISharedEventPipeStreamData sharedData,
  int threadId,
  string? targetMethodsRegex)
{
  private readonly Regex? myTargetMethodsRegex = targetMethodsRegex is { } ? new Regex(targetMethodsRegex) : null;
  private readonly Stack<TargetMethodFrame> myMethodsStack = new();


  public void Process(TraceEvent traceEvent)
  {
    var qpcStamp = (long)traceEvent.PayloadValue(0);
    var methodId = (ulong)traceEvent.PayloadValue(1);
    var isTargetMethod = IsTargetMethod(methodId);

    switch (traceEvent.GetMethodEventKind())
    {
      case MethodKind.Begin:
      {
        if (isTargetMethod)
        {
          myMethodsStack.Push(new TargetMethodFrame(methodId));
        }

        foreach (var targetFrame in myMethodsStack)
        {
          targetFrame.InnerFrames.Add(new MethodFrame(true, methodId, qpcStamp));
        }

        break;
      }
      case MethodKind.End:
      {
        foreach (var targetFrame in myMethodsStack)
        {
          targetFrame.InnerFrames.Add(new MethodFrame(false, methodId, qpcStamp));
        }

        if (isTargetMethod)
        {
          if (methodId != myMethodsStack.Peek().MethodId)
          {
            logger.LogWarning("The stack is corrupt for thread {ThreadId}", threadId);
          }

          handler.Handle(new CompletedMethodExecutionEvent
          {
            Frame = myMethodsStack.Pop()
          });
        }

        break;
      }
      default:
        throw new ArgumentOutOfRangeException();
    }
  }

  private bool IsTargetMethod(ulong methodId)
  {
    if (sharedData.FindMethodFqn(methodId) is not { } methodFqn) return false;

    return myTargetMethodsRegex is null || myTargetMethodsRegex.IsMatch(methodFqn);
  }
}