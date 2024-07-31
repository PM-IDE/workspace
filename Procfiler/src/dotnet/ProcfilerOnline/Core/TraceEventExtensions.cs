using Core.Events.EventRecord;

namespace ProcfilerOnline.Core;

public enum MethodKind
{
  Begin,
  End
}

public static class OnlineProcfilerConstants
{
  public const string CppMethodStartEventName = "ProcfilerMethod/Begin";
  public const string CppMethodFinishedEventName = "ProcfilerMethod/End";

  public const string FunctionId = "FunctionId";
  public const string Timestamp = "Timestamp";
}

public static class TraceEventExtensions
{
  public static MethodKind GetMethodEventKind(this EventRecordWithMetadata eventRecord) => eventRecord.EventClass switch
  {
    OnlineProcfilerConstants.CppMethodStartEventName => MethodKind.Begin,
    OnlineProcfilerConstants.CppMethodFinishedEventName => MethodKind.End,
    _ => throw new ArgumentOutOfRangeException()
  };

  public static (long QpcStamp, long methodId)? TryGetMethodDetails(this EventRecordWithMetadata traceEvent)
  {
    if (traceEvent.EventClass is OnlineProcfilerConstants.CppMethodFinishedEventName or OnlineProcfilerConstants.CppMethodStartEventName)
    {
      var qpcStamp = traceEvent.Metadata[OnlineProcfilerConstants.Timestamp];
      var methodId = traceEvent.Metadata[OnlineProcfilerConstants.FunctionId];
      return (long.Parse(qpcStamp), long.Parse(methodId));
    }

    return null;
  }
}