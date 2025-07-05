using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection;
using Procfiler.Core.Serialization.Core;

namespace Procfiler.Core.SplitByMethod;

public abstract record EventUpdateBase(CurrentFrameInfo FrameInfo);

public sealed record MethodStartedUpdate(CurrentFrameInfo FrameInfo) : EventUpdateBase(FrameInfo);

public sealed record MethodFinishedUpdate(CurrentFrameInfo FrameInfo) : EventUpdateBase(FrameInfo);

public sealed record MethodExecutionUpdate(CurrentFrameInfo FrameInfo, string MethodName) : EventUpdateBase(FrameInfo);

public sealed record NormalEventUpdate(CurrentFrameInfo FrameInfo, EventRecordWithMetadata Event) : EventUpdateBase(FrameInfo);

public enum EventKind
{
  MethodStarted,
  MethodFinished,
  MethodExecution,
  Normal
}

public class CallbackBasedSplitter(
  IProcfilerLogger logger,
  IEnumerable<EventRecordWithPointer> events,
  string filterPattern,
  InlineMode inlineMode,
  IOnlineMethodsSerializer serializer
)
{
  private readonly Stack<CurrentFrameInfo> myFramesStack = new();
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
    var state = serializer.CreateState(eventRecord);
    var shouldProcess = ShouldProcess(frame);

    var frameInfo = new CurrentFrameInfo(
      frame, shouldProcess, eventRecord.Time, eventRecord.ManagedThreadId, eventRecord.NativeThreadId, state);

    serializer.HandleUpdate(new MethodStartedUpdate(frameInfo));
    serializer.HandleUpdate(new NormalEventUpdate(frameInfo, eventRecord));

    if (ShouldInline(frame))
    {
      ExecuteCallbackForAllFrames(EventKind.Normal, eventRecord);
    }

    myFramesStack.Push(frameInfo);
  }

  private bool ShouldInline(string frame) =>
    inlineMode == InlineMode.EventsAndMethodsEvents ||
    inlineMode == InlineMode.EventsAndMethodsEventsWithFilter && ShouldProcess(frame);

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

    serializer.HandleUpdate(new NormalEventUpdate(topOfStack, methodEndEvent));
    serializer.HandleUpdate(new MethodFinishedUpdate(topOfStack));

    if (ShouldInline(frame))
    {
      ExecuteCallbackForAllFrames(EventKind.Normal, methodEndEvent);
      return;
    }

    if (myFramesStack.Count <= 0) return;

    serializer.HandleUpdate(new MethodExecutionUpdate(myFramesStack.Peek(), topOfStack.Frame));
  }

  private void ExecuteCallbackForAllFrames(EventKind eventKind, EventRecordWithMetadata eventRecord)
  {
    foreach (var frameInfo in myFramesStack)
    {
      if (frameInfo.ShouldProcess)
      {
        EventUpdateBase update = eventKind switch
        {
          EventKind.MethodStarted => new MethodStartedUpdate(frameInfo),
          EventKind.MethodFinished => new MethodFinishedUpdate(frameInfo),
          EventKind.Normal => new NormalEventUpdate(frameInfo, eventRecord),
          _ => throw new ArgumentOutOfRangeException(nameof(eventKind), eventKind, null)
        };

        serializer.HandleUpdate(update);
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
        serializer.HandleUpdate(new NormalEventUpdate(topmostFrame, eventRecord));
      }
    }
  }
}