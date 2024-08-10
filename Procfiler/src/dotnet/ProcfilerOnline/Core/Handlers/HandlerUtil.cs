using Core.Events.EventRecord;

namespace ProcfilerOnline.Core.Handlers;

public static class HandlerUtil
{
  public static (string, bool)? ExtractFrame(EventRecordWithMetadata eventRecord) => eventRecord.TryGetMethodDetails() switch
  {
    { } => (eventRecord.EventName, eventRecord.GetMethodEventKind() == MethodKind.Begin),
    _ => null
  };
}