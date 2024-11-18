using Core.Events.EventRecord;
using Procfiler.Core.EventRecord;

namespace Procfiler.Core.SplitByMethod;

public readonly record struct CurrentFrameInfo<T>(
  string Frame,
  bool ShouldProcess,
  EventRecordTime OriginalEventTime,
  long ManagedThreadId,
  long NativeThreadId,
  T? State
);

public static class CurrentFrameInfoUtil
{
  public static EventRecordWithMetadata CreateMethodExecutionEvent<T>(
    CurrentFrameInfo<T> frameInfo, IProcfilerEventsFactory factory, string methodName, EventRecordWithMetadata? contextEvent)
  {
    var startEventCtx = contextEvent switch
    {
      { } => EventsCreationContext.CreateWithUndefinedStackTrace(contextEvent),
      _ => new EventsCreationContext(frameInfo.OriginalEventTime, frameInfo.ManagedThreadId, frameInfo.NativeThreadId)
    };

    return factory.CreateMethodExecutionEvent(startEventCtx, methodName);
  }
}