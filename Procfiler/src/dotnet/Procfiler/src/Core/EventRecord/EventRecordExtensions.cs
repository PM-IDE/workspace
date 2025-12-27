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

    public string GetMethodNameOrThrow() => eventRecord.Metadata[TraceEventsConstants.ProcfilerMethodName];

    public MethodStartEndEventInfo? TryGetMethodStartEndEventInfo()
    {
      if (eventRecord.IsMethodStartOrEndEvent())
      {
        return new MethodStartEndEventInfo(
          eventRecord.GetMethodNameOrThrow(),
          eventRecord.EventClass is TraceEventsConstants.ProcfilerMethodStart
        );
      }

      return null;
    }

    public bool IsMethodExecutionEvent(out string? methodName)
    {
      methodName = null;
      if (eventRecord.EventClass is not TraceEventsConstants.ProcfilerMethodExecution) return false;

      methodName = eventRecord.GetMethodNameOrThrow();
      return true;
    }
  }
}