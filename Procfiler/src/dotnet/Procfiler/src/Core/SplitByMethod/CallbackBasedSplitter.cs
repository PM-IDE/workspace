using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection;
using Procfiler.Core.Serialization.Core;

namespace Procfiler.Core.SplitByMethod;

public abstract record EventUpdateBase(CurrentFrameInfoWithState FrameInfo);

public sealed record MethodStartedUpdate(CurrentFrameInfoWithState FrameInfo) : EventUpdateBase(FrameInfo);

public sealed record MethodFinishedUpdate(CurrentFrameInfoWithState FrameInfo) : EventUpdateBase(FrameInfo);

public sealed record MethodExecutionUpdate(CurrentFrameInfoWithState FrameInfo, string MethodName) : EventUpdateBase(FrameInfo);

public sealed record NormalEventUpdate(CurrentFrameInfoWithState FrameInfo, EventRecordWithMetadata Event) : EventUpdateBase(FrameInfo);

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
  List<IOnlineMethodsSerializer> serializers
)
{
  private readonly Stack<(CurrentFrameInfo Frame, List<object?> States)> myFramesStack = new();
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
    var states = serializers.Select(s => s.CreateState(eventRecord)).ToList();
    var shouldProcess = ShouldProcess(frame);

    var frameInfo = new CurrentFrameInfo(
      frame, shouldProcess, eventRecord.Time, eventRecord.ManagedThreadId, eventRecord.NativeThreadId);

    foreach (var (serializer, state) in serializers.Zip(states))
    {
      var info = new CurrentFrameInfoWithState(frameInfo, state);
      serializer.HandleUpdate(new MethodStartedUpdate(info));
      serializer.HandleUpdate(new NormalEventUpdate(info, eventRecord));
    }

    if (ShouldInline(frame))
    {
      ExecuteCallbackForAllFrames(EventKind.Normal, eventRecord);
    }

    myFramesStack.Push((frameInfo, states));
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

    var (topOfStack, states) = myFramesStack.Pop();
    if (!topOfStack.ShouldProcess) return;

    foreach (var (serializer, state) in serializers.Zip(states))
    {
      var info = new CurrentFrameInfoWithState(topOfStack, state);
      serializer.HandleUpdate(new NormalEventUpdate(info, methodEndEvent));
      serializer.HandleUpdate(new MethodFinishedUpdate(info));
    }

    if (ShouldInline(frame))
    {
      ExecuteCallbackForAllFrames(EventKind.Normal, methodEndEvent);
      return;
    }

    if (myFramesStack.Count <= 0) return;

    (topOfStack, states) = myFramesStack.Peek();
    foreach (var (serializer, state) in serializers.Zip(states))
    {
      serializer.HandleUpdate(new MethodExecutionUpdate(new CurrentFrameInfoWithState(topOfStack, state), topOfStack.Frame));
    }
  }

  private void ExecuteCallbackForAllFrames(EventKind eventKind, EventRecordWithMetadata eventRecord)
  {
    foreach (var (frameInfo, states) in myFramesStack)
    {
      if (!frameInfo.ShouldProcess) continue;

      foreach (var (serializer, state) in serializers.Zip(states))
      {
        var info = new CurrentFrameInfoWithState(frameInfo, state);
        EventUpdateBase update = eventKind switch
        {
          EventKind.MethodStarted => new MethodStartedUpdate(info),
          EventKind.MethodFinished => new MethodFinishedUpdate(info),
          EventKind.Normal => new NormalEventUpdate(info, eventRecord),
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
      var (topmostFrame, states) = myFramesStack.Peek();
      if (topmostFrame.ShouldProcess)
      {
        foreach (var (serializer, state) in serializers.Zip(states))
        {
          serializer.HandleUpdate(new NormalEventUpdate(new CurrentFrameInfoWithState(topmostFrame, state), eventRecord));
        }
      }
    }
  }
}