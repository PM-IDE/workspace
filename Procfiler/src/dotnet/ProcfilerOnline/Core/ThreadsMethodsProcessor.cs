﻿using System.Text.RegularExpressions;
using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Core.Handlers;
using ProcfilerOnline.Core.Mutators;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core;

public interface IThreadsMethodsProcessor
{
  void Process(EventProcessingContext context);
  IReadOnlyList<(long ThreadId, List<EventRecordWithMetadata>)> ReclaimNotClosedMethods();
}

public class TargetMethodFrame(long methodId, ExtendedMethodInfo? methodInfo)
{
  public long MethodId { get; } = methodId;
  public ExtendedMethodInfo? MethodInfo { get; } = methodInfo;
  public Guid CaseId { get; } = Guid.NewGuid();

  public List<EventRecordWithMetadata> InnerEvents { get; } = [];
}

[AppComponent]
public class ThreadsMethodsProcessor(
  IProcfilerLogger logger,
  ICompositeEventPipeStreamEventHandler handler,
  IEventProcessingEntryPoint eventProcessingEntryPoint,
  IMethodBeginEndSingleMutator methodBeginEndSingleMutator
) : IThreadsMethodsProcessor
{
  private readonly Dictionary<long, Stack<TargetMethodFrame>> myStacksPerThreads = new();


  public void Process(EventProcessingContext context)
  {
    if (TryProcessExceptionCatcherEnterEvent(context)) return;

    ProcessInternal(context);
    FlushMethods(context);
  }

  private void FlushMethods(EventProcessingContext context)
  {
    foreach (var (_, threadStack) in myStacksPerThreads.ToList())
    {
      foreach (var frame in threadStack)
      {
        var eventsCount = (ulong)frame.InnerEvents.Count;
        if (eventsCount <= context.CommandContext.EventsFlushThreshold) continue;

        logger.LogInformation(
          "Flushing method {MethodName} as events count {EventsCount} exceeds threshold {FlushThreshold}",
          frame.MethodInfo?.Fqn,
          eventsCount,
          context.CommandContext.EventsFlushThreshold
        );

        handler.Handle(new MethodExecutionEvent
        {
          Frame = frame,
          ApplicationName = context.CommandContext.ApplicationName
        });

        frame.InnerEvents.Clear();
      }
    }
  }

  private bool TryProcessExceptionCatcherEnterEvent(EventProcessingContext context)
  {
    if (!context.Event.IsExceptionCatcherEnter(out var functionId)) return false;

    var threadStack = GetOrCreateThreadStack(context.Event.NativeThreadId);

    while (threadStack.Count > 0 && threadStack.Peek().MethodId != functionId)
    {
      ProcessInternal(context with
      {
        Event = threadStack.Peek().InnerEvents.First().ConvertToMethodEndEvent(context.SharedData, methodBeginEndSingleMutator)
      });
    }

    return true;
  }

  private Stack<TargetMethodFrame> GetOrCreateThreadStack(long threadId) =>
    myStacksPerThreads.GetOrCreate(threadId, static () => new Stack<TargetMethodFrame>());

  private void ProcessInternal(EventProcessingContext context)
  {
    var eventRecord = context.Event;
    var threadId = eventRecord.NativeThreadId;
    var threadStack = GetOrCreateThreadStack(threadId);

    foreach (var targetFrame in threadStack)
    {
      targetFrame.InnerEvents.Add(eventRecord);
    }

    eventProcessingEntryPoint.Process(context);

    ProcessMethodStartEndEvent(context);
  }

  private void ProcessMethodStartEndEvent(EventProcessingContext context)
  {
    if (context.Event.TryGetMethodDetails() is not var (_, methodId)) return;

    var threadStack = GetOrCreateThreadStack(context.Event.NativeThreadId);
    var isTargetMethod = IsTargetMethod(context, methodId, context.CommandContext.TargetMethodsRegex);

    switch (context.Event.GetMethodEventKind())
    {
      case MethodKind.Begin:
      {
        if (isTargetMethod)
        {
          var methodName = context.SharedData.FindMethodDetails(methodId);
          threadStack.Push(new TargetMethodFrame(methodId, methodName));
          threadStack.Peek().InnerEvents.Add(context.Event);
        }

        break;
      }
      case MethodKind.End:
      {
        if (isTargetMethod)
        {
          if (methodId != threadStack.Peek().MethodId)
          {
            logger.LogWarning("The stack is corrupt for thread {ThreadId}", context.Event.NativeThreadId);
          }

          var frame = threadStack.Pop();

          if (context.SharedData.FindMethodDetails(methodId) is not { Fqn: var fqn }) return;

          if (context.CommandContext.TargetMethodsRegex is null ||
              context.CommandContext.TargetMethodsRegex.IsMatch(fqn))
          {
            handler.Handle(new MethodExecutionEvent
            {
              ApplicationName = context.CommandContext.ApplicationName,
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

  public IReadOnlyList<(long ThreadId, List<EventRecordWithMetadata>)> ReclaimNotClosedMethods()
  {
    return myStacksPerThreads
      .Where(pair => pair.Value.Count > 0)
      .Select(pair => (pair.Key, pair.Value.Select(targetFrame => targetFrame.InnerEvents.First()).ToList()))
      .ToList();
  }

  private bool IsTargetMethod(EventProcessingContext context, long methodId, Regex? targetMethodsRegex)
  {
    if (context.SharedData.FindMethodDetails(methodId) is not { Fqn: var fqn }) return false;

    return targetMethodsRegex is null || targetMethodsRegex.IsMatch(fqn);
  }
}