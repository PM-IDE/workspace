using Procfiler.Core.EventRecord;

namespace Procfiler.Core.SplitByMethod;

public readonly record struct CurrentFrameInfo<T>(
  string Frame,
  bool ShouldProcess,
  long OriginalEventStamp,
  long OriginalEventThreadId,
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
      _ => new EventsCreationContext(frameInfo.OriginalEventStamp, frameInfo.OriginalEventThreadId)
    };

    return factory.CreateMethodExecutionEvent(startEventCtx, methodName);
  }
}