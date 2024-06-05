using Procfiler.Core.Collector;
using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsCollection;
using Procfiler.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.MultipleEventsMutators;

public class FromEventsMethodsStartEndMutator(
  IProcfilerEventsFactory eventsFactory,
  IProcfilerLogger logger) : IMethodsStartEndProcessor
{
  private record StackFrameInfo(string Frame, EventRecordWithMetadata StartNode)
  {
    public string Frame { get; private set; } = Frame;


    public void UpdateFrame(string newFrame)
    {
      Frame = newFrame;
      StartNode.Metadata[TraceEventsConstants.ProcfilerMethodName] = newFrame;
    }
  }


  public void Process(IEventsCollection events, SessionGlobalData context)
  {
    if (context.Stacks is not IFromEventsShadowStacks fromEventsShadowStacks)
    {
      var name = context.Stacks.GetType().Name;
      logger.LogError("Not compatible shadow stacks, got {Type}, expected {Type}", name, nameof(IFromEventsShadowStacks));

      return;
    }

    var currentFrames = new List<StackFrameInfo>();
    EventRecordWithPointer? last = null;
    events.ApplyNotPureActionForAllEvents(eventWithPtr =>
    {
      var (ptr, eventRecord) = eventWithPtr;
      last = eventWithPtr;

      if (!eventRecord.IsMethodStartEndProvider()) return false;
      if (!fromEventsShadowStacks.StackTraceInfos.TryGetValue(eventRecord.StackTraceId, out var stack)) return false;

      if (currentFrames.Count == 0)
      {
        InitializeCurrentStack(currentFrames, stack, events, ptr, eventRecord);
        return false;
      }

      var indexOfFirstDifferentFrame = GetIndexWhereStacksDiffer(currentFrames, stack);
      switch (indexOfFirstDifferentFrame)
      {
        case -1 when currentFrames.Count == stack.Frames.Length:
          break;

        case -1 when currentFrames.Count > stack.Frames.Length:
          PopEventsFromCurrentStack(currentFrames, events, ptr, eventRecord, indexOfFirstDifferentFrame);
          break;

        case -1:
          AddFramesFromEventToCurrentStack(currentFrames, stack, events, ptr, eventRecord);
          break;

        default:
          PopEventsFromCurrentStack(currentFrames, events, ptr, eventRecord, indexOfFirstDifferentFrame);
          AddFramesFromEventToCurrentStack(currentFrames, stack, events, ptr, eventRecord);
          break;
      }

      return false;
    });

    if (last is null) return;

    var lastPtr = last.Value.EventPointer;

    while (currentFrames.Count != 0)
    {
      var info = currentFrames[^1];
      currentFrames.RemoveAt(currentFrames.Count - 1);

      var lastEvent = last!.Value.Event;
      lastPtr = events.InsertAfter(lastPtr, CreateMethodEndEvent(lastEvent, info.Frame));
    }
  }

  private void InitializeCurrentStack(
    List<StackFrameInfo> currentFrames,
    StackTraceInfo stack,
    IEventsCollection events,
    EventPointer pointer,
    EventRecordWithMetadata current)
  {
    for (var i = stack.Frames.Length - 1; i >= 0; i--)
    {
      var frame = stack.Frames[i];
      var methodStartEvent = CreateMethodStartEvent(current, frame);
      currentFrames.Add(new StackFrameInfo(frame, methodStartEvent));

      events.InsertBefore(pointer, methodStartEvent);
    }
  }

  private static int GetIndexWhereStacksDiffer(List<StackFrameInfo> currentFrames, StackTraceInfo stack)
  {
    var indexOfFirstDifferentFrame = -1;
    var index = 0;
    foreach (var currentFrame in currentFrames)
    {
      if (index >= stack.Frames.Length)
      {
        indexOfFirstDifferentFrame = index;
        break;
      }

      if (!StackFramesEqual(currentFrame.Frame, stack.Frames[stack.Frames.Length - 1 - index]))
      {
        indexOfFirstDifferentFrame = index;
        break;
      }

      ++index;
    }

    return indexOfFirstDifferentFrame;
  }

  private static bool StackFramesEqual(string firstFrame, string secondFrame)
  {
    if (firstFrame is TraceEventsConstants.UndefinedMethod ||
        secondFrame is TraceEventsConstants.UndefinedMethod)
    {
      return true;
    }

    return firstFrame == secondFrame;
  }

  private void PopEventsFromCurrentStack(
    List<StackFrameInfo> currentFrames,
    IEventsCollection events,
    EventPointer current,
    EventRecordWithMetadata eventRecord,
    int indexOfFirstDifferentFrame)
  {
    while (currentFrames.Count != indexOfFirstDifferentFrame)
    {
      var lastFrame = currentFrames[^1];
      currentFrames.RemoveAt(currentFrames.Count - 1);
      events.InsertBefore(current, CreateMethodEndEvent(eventRecord, lastFrame.Frame));
    }
  }

  private EventRecordWithMetadata CreateMethodEndEvent(EventRecordWithMetadata @event, string frame) =>
    eventsFactory.CreateMethodEndEvent(CreateContext(@event), frame);

  private static EventsCreationContext CreateContext(EventRecord.EventRecord @event) =>
    new(@event.Time, @event.ManagedThreadId);

  private EventRecordWithMetadata CreateMethodStartEvent(EventRecordWithMetadata @event, string frame) =>
    eventsFactory.CreateMethodStartEvent(CreateContext(@event), frame);

  private void AddFramesFromEventToCurrentStack(
    List<StackFrameInfo> currentFrames,
    StackTraceInfo stack,
    IEventsCollection events,
    EventPointer current,
    EventRecordWithMetadata contextEvent)
  {
    for (var index = stack.Frames.Length - 1 - currentFrames.Count; index >= 0; --index)
    {
      var frameToAdd = stack.Frames[index];
      var methodStartEvent = CreateMethodStartEvent(contextEvent, frameToAdd);
      currentFrames.Add(new StackFrameInfo(frameToAdd, methodStartEvent));

      events.InsertBefore(current, methodStartEvent);
    }
  }
}