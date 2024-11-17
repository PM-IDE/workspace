using Core.Events.EventRecord;
using ProcfilerOnline.Core.Mutators;

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
  public const string ExceptionCatcherEnterEventName = "ExceptionCatcher/Enter";
  public const string ManagedThreadToNativeAssignment = "ManagedThreadToNativeAssignment";

  public const string FunctionId = "FunctionId";
  public const string Timestamp = "Timestamp";

  public const string ManagedThreadId = "ManagedThreadId";
  public const string NativeThreadId = "NativeThreadId";
}

public static class TraceEventExtensions
{
  public static MethodKind GetMethodEventKind(this EventRecordWithMetadata eventRecord) => eventRecord.EventClass switch
  {
    OnlineProcfilerConstants.CppMethodStartEventName => MethodKind.Begin,
    OnlineProcfilerConstants.CppMethodFinishedEventName => MethodKind.End,
    _ => throw new ArgumentOutOfRangeException()
  };

  public static (long QpcStamp, long MethodId)? TryGetMethodDetails(this EventRecordWithMetadata eventRecord)
  {
    if (eventRecord.EventClass is OnlineProcfilerConstants.CppMethodFinishedEventName
        or OnlineProcfilerConstants.CppMethodStartEventName)
    {
      var qpcStamp = eventRecord.Metadata[OnlineProcfilerConstants.Timestamp];
      var methodId = eventRecord.Metadata[OnlineProcfilerConstants.FunctionId];
      return (long.Parse(qpcStamp), long.Parse(methodId));
    }

    return null;
  }

  public static bool IsExceptionCatcherEnter(this EventRecordWithMetadata eventRecord, out long functionId)
  {
    functionId = -1;

    if (eventRecord.EventClass is not OnlineProcfilerConstants.ExceptionCatcherEnterEventName) return false;

    functionId = long.Parse(eventRecord.Metadata[OnlineProcfilerConstants.FunctionId]);
    return true;
  }

  public static EventRecordWithMetadata ConvertToMethodEndEvent(
    this EventRecordWithMetadata eventRecord, ISharedEventPipeStreamData globalData, IMethodBeginEndSingleMutator mutator)
  {
    if (eventRecord.EventClass is not OnlineProcfilerConstants.CppMethodStartEventName)
    {
      throw new ArgumentOutOfRangeException();
    }

    var methodEvent = eventRecord.DeepClone();
    methodEvent.EventClass = OnlineProcfilerConstants.CppMethodFinishedEventName;

    mutator.Process(methodEvent, globalData);

    return methodEvent;
  }
}