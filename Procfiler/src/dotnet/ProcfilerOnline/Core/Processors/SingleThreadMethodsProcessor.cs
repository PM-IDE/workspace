﻿using System.Text.RegularExpressions;
using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Core.Handlers;

namespace ProcfilerOnline.Core.Processors;

public class TargetMethodFrame(long methodId)
{
  public long MethodId { get; } = methodId;
  public List<EventRecordWithMetadata> InnerEvents { get; } = [];
}

[AppComponent]
public class SingleThreadMethodsProcessor(
  IProcfilerLogger logger,
  ICompositeEventPipeStreamEventHandler handler
) : ITraceEventProcessor
{
  private readonly Dictionary<long, Stack<TargetMethodFrame>> myStacksPerThreads = new();


  public void Process(EventProcessingContext context)
  {
    var eventRecord = context.Event;
    var threadId = eventRecord.ManagedThreadId;
    var threadStack = myStacksPerThreads.GetOrCreate(threadId, static () => new Stack<TargetMethodFrame>());

    foreach (var targetFrame in threadStack)
    {
      targetFrame.InnerEvents.Add(eventRecord);
    }

    if (eventRecord.TryGetMethodDetails() is not var (_, methodId)) return;

    var isTargetMethod = IsTargetMethod(context, methodId, context.CommandContext.TargetMethodsRegex);

    switch (eventRecord.GetMethodEventKind())
    {
      case MethodKind.Begin:
      {
        if (isTargetMethod)
        {
          threadStack.Push(new TargetMethodFrame(methodId));
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

          if (!context.SharedData.MethodIdToFqn.TryGetValue(frame.MethodId, out var methodFqn)) return;

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

  private bool IsTargetMethod(EventProcessingContext context, long methodId, Regex? targetMethodsRegex)
  {
    if (!context.SharedData.MethodIdToFqn.TryGetValue(methodId, out var methodFqn)) return false;

    return targetMethodsRegex is null || targetMethodsRegex.IsMatch(methodFqn);
  }
}