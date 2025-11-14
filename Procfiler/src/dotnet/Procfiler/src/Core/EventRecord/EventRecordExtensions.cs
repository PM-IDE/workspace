using Core.Constants.TraceEvents;
using Core.Events.EventRecord;

namespace Procfiler.Core.EventRecord;

public static class EventRecordExtensions
{
  public readonly record struct MethodStartEndEventInfo(string Frame, bool IsStart);


  extension(EventRecordWithMetadata eventRecord)
  {
    public bool IsMethodStartOrEndEvent() =>
      eventRecord.EventClass is TraceEventsConstants.ProcfilerMethodStart or TraceEventsConstants.ProcfilerMethodEnd;

    public MethodStartEndEventInfo GetMethodStartEndEventInfo()
      => eventRecord.TryGetMethodStartEndEventInfo() ?? throw new ArgumentOutOfRangeException();

    public MethodStartEndEventInfo? TryGetMethodStartEndEventInfo()
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
}