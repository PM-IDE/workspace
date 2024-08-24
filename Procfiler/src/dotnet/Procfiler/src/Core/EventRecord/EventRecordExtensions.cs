using Core.Constants.TraceEvents;
using Core.Events.EventRecord;

namespace Procfiler.Core.EventRecord;

public static class EventRecordExtensions
{
  public readonly record struct MethodStartEndEventInfo(string Frame, bool IsStart);


  public static bool IsMethodStartOrEndEvent(this EventRecordWithMetadata eventRecord) =>
    eventRecord.EventClass is TraceEventsConstants.ProcfilerMethodStart or TraceEventsConstants.ProcfilerMethodEnd;


  public static MethodStartEndEventInfo GetMethodStartEndEventInfo(this EventRecordWithMetadata eventRecord)
    => eventRecord.TryGetMethodStartEndEventInfo() ?? throw new ArgumentOutOfRangeException();

  public static MethodStartEndEventInfo? TryGetMethodStartEndEventInfo(this EventRecordWithMetadata eventRecord)
  {
    if (IsMethodStartOrEndEvent(eventRecord))
    {
      return new MethodStartEndEventInfo(
        eventRecord.Metadata[TraceEventsConstants.ProcfilerMethodName],
        eventRecord.EventClass is TraceEventsConstants.ProcfilerMethodStart
      );
    }

    return null;
  }
}