using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsCollection;
using Procfiler.Utils;

namespace Procfiler.Core.SplitByMethod;

public abstract record EventUpdateBase<T>(CurrentFrameInfo<T> FrameInfo);

public sealed record MethodStartedUpdate<T>(CurrentFrameInfo<T> FrameInfo, EventRecordWithMetadata Event) : EventUpdateBase<T>(FrameInfo);

public sealed record MethodFinishedUpdate<T>(CurrentFrameInfo<T> FrameInfo, EventRecordWithMetadata Event) : EventUpdateBase<T>(FrameInfo);

public sealed record MethodExecutionUpdate<T>(CurrentFrameInfo<T> FrameInfo, string MethodName) : EventUpdateBase<T>(FrameInfo);

public sealed record NormalEventUpdate<T>(CurrentFrameInfo<T> FrameInfo, EventRecordWithMetadata Event) : EventUpdateBase<T>(FrameInfo);

public enum EventKind
{
  MethodStarted,
  MethodFinished,
  MethodExecution,
  Normal
}

public class CallbackBasedSplitter<T>(
  IProcfilerLogger logger,
  IEnumerable<EventRecordWithPointer> events,
  string filterPattern,
  InlineMode inlineMode,
  Func<EventRecordWithMetadata, T?> stateFactory,
  Action<EventUpdateBase<T>> callback)
{
  private readonly Stack<CurrentFrameInfo<T>> myFramesStack = new();
  private readonly Regex myFilterRegex = new(filterPattern);

  public void Split()
  {
    foreach (var (_, eventRecord) in events)
    {
      if (eventRecord.TryGetMethodStartEndEventInfo() is var (frame, isStartOfMethod))
      {
        if (isStartOfMethod)
        {
          ProcessStartOfMethod(frame, eventRecord);
          continue;
        }

        ProcessEndOfMethod(frame, eventRecord);
        continue;
      }

      ProcessNormalEvent(eventRecord);
    }
  }

  private void ProcessStartOfMethod(string frame, EventRecordWithMetadata eventRecord)
  {
    var state = stateFactory(eventRecord);
    var frameInfo = new CurrentFrameInfo<T>(frame, ShouldProcess(frame), eventRecord.Stamp, eventRecord.ManagedThreadId, state);
    callback(new MethodStartedUpdate<T>(frameInfo, eventRecord));
    callback(new NormalEventUpdate<T>(frameInfo, eventRecord));

    if (ShouldInline(frame))
    {
      ExecuteCallbackForAllFrames(EventKind.Normal, eventRecord);
    }

    myFramesStack.Push(frameInfo);
  }

  private bool ShouldInline(string frame) =>
    inlineMode == InlineMode.EventsAndMethodsEvents ||
    (inlineMode == InlineMode.EventsAndMethodsEventsWithFilter && ShouldProcess(frame));

  private bool ShouldProcess(string frame) => myFilterRegex.IsMatch(frame);

  private void ProcessEndOfMethod(string frame, EventRecordWithMetadata methodEndEvent)
  {
    if (myFramesStack.Count == 0)
    {
      logger.LogWarning("Broken stacks: method {Name} ended without starting", frame);
      return;
    }

    var topOfStack = myFramesStack.Pop();
    if (!topOfStack.ShouldProcess) return;

    callback(new NormalEventUpdate<T>(topOfStack, methodEndEvent));
    callback(new MethodFinishedUpdate<T>(topOfStack, methodEndEvent));

    if (ShouldInline(frame))
    {
      ExecuteCallbackForAllFrames(EventKind.Normal, methodEndEvent);
      return;
    }

    if (myFramesStack.Count <= 0) return;

    callback(new MethodExecutionUpdate<T>(myFramesStack.Peek(), topOfStack.Frame));
  }

  private void ExecuteCallbackForAllFrames(EventKind eventKind, EventRecordWithMetadata eventRecord)
  {
    foreach (var frameInfo in myFramesStack)
    {
      if (frameInfo.ShouldProcess)
      {
        EventUpdateBase<T> update = eventKind switch
        {
          EventKind.MethodStarted => new MethodStartedUpdate<T>(frameInfo, eventRecord),
          EventKind.MethodFinished => new MethodFinishedUpdate<T>(frameInfo, eventRecord),
          EventKind.Normal => new NormalEventUpdate<T>(frameInfo, eventRecord),
          _ => throw new ArgumentOutOfRangeException(nameof(eventKind), eventKind, null)
        };

        callback(update);
      }
    }
  }

  private void ProcessNormalEvent(EventRecordWithMetadata eventRecord)
  {
    if (myFramesStack.Count <= 0) return;

    if (inlineMode != InlineMode.NotInline)
    {
      ExecuteCallbackForAllFrames(EventKind.Normal, eventRecord);
    }
    else
    {
      var topmostFrame = myFramesStack.Peek();
      if (topmostFrame.ShouldProcess)
      {
        callback(new NormalEventUpdate<T>(topmostFrame, eventRecord));
      }
    }
  }
}