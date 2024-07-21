using Core.Utils;
using Microsoft.Diagnostics.Tracing;
using Microsoft.Extensions.Logging;

namespace ProcfilerOnline.Core;

public readonly record struct MethodFrame(bool IsStart, ulong MethodId, long QpcStamp);

public class TargetMethodFrame(ulong methodId)
{
  public ulong MethodId { get; } = methodId;
  public List<MethodFrame> InnerFrames { get; } = [];
}

public class ThreadEventsProcessor(IProcfilerLogger logger, int threadId)
{
  private readonly Stack<TargetMethodFrame> myMethodsStack = new();


  public void Process(TraceEvent traceEvent)
  {
    var qpcStamp = (long)traceEvent.PayloadValue(0);
    var methodId = (ulong)traceEvent.PayloadValue(1);

    switch (traceEvent.GetMethodEventKind())
    {
      case MethodKind.Begin:
      {
        myMethodsStack.Push(new TargetMethodFrame(methodId));
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

        if (methodId != myMethodsStack.Peek().MethodId)
        {
          logger.LogWarning("The stack is corrupt for thread {ThreadId}", threadId);
        }

        myMethodsStack.Pop();

        break;
      }
      default:
        throw new ArgumentOutOfRangeException();
    }
  }
}