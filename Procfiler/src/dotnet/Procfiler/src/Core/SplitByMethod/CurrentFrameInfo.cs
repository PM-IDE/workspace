using Core.Events.EventRecord;
using Procfiler.Core.EventRecord;

namespace Procfiler.Core.SplitByMethod;

public readonly record struct CurrentFrameInfo(
  string Frame,
  bool ShouldProcess,
  EventRecordTime OriginalEventTime,
  long ManagedThreadId,
  long NativeThreadId
);

public readonly record struct CurrentFrameInfoWithState(
  CurrentFrameInfo Frame,
  object? State
);

public static class CurrentFrameInfoUtil
{
  public static EventRecordWithMetadata CreateMethodExecutionEvent(
    CurrentFrameInfo frameInfo, IProcfilerEventsFactory factory, string methodName, EventRecordWithMetadata? contextEvent)
  {
    var startEventCtx = contextEvent switch
    {
      { } => EventsCreationContext.CreateWithUndefinedStackTrace(contextEvent),
      _ => new EventsCreationContext(frameInfo.OriginalEventTime, frameInfo.ManagedThreadId, frameInfo.NativeThreadId)
    };

    return factory.CreateMethodExecutionEvent(startEventCtx, methodName);
  }
}