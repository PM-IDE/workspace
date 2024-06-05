using Procfiler.Core.Collector;
using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventRecord;

namespace Procfiler.Core;

internal static class TraceEventsExtensions
{
  public static EventRecordTime ToTime(this TraceEvent traceEvent) => new()
  {
    LoggedAt = traceEvent.TimeStamp,
    QpcStamp = traceEvent.TimeStampQPC,
    RelativeStampMSec = traceEvent.TimeStampRelativeMSec
  };

  public static Dictionary<string, string> CreateMetadataMap(this TraceEvent traceEvent)
  {
    var metadata = new Dictionary<string, string>();
    for (var index = 0; index < traceEvent.PayloadNames.Length; index++)
    {
      var name = traceEvent.PayloadNames[index];
      metadata[name] = string.Intern(traceEvent.PayloadString(index));
    }

    return metadata;
  }

  public static bool IsThreadStartMethod(string frame, out int threadId)
  {
    threadId = -1;
    if (!frame.Contains(TraceEventsConstants.ThreadFrameTemplate)) return false;

    var from = TraceEventsConstants.ThreadFrameTemplate.Length;
    var to = frame.IndexOf(')');
    if (!int.TryParse(frame.AsSpan(from, to - from), out threadId))
    {
      threadId = -1;
    }

    return true;
  }

  public static int GetManagedThreadIdThroughStack(
    this TraceEvent @event,
    MutableTraceEventStackSource stackSource)
  {
    var callStackIndex = @event.CallStackIndex();
    if (callStackIndex is CallStackIndex.Invalid) return -1;

    var currentIndex = stackSource.GetCallStack(callStackIndex, @event);
    var managedThreadId = -1;

    while (currentIndex != StackSourceCallStackIndex.Invalid)
    {
      var frame = string.Intern(stackSource.GetFrameName(stackSource.GetFrameIndex(currentIndex), false));
      if (IsThreadStartMethod(frame, out var threadId))
      {
        managedThreadId = threadId;
      }

      currentIndex = stackSource.GetCallerIndex(currentIndex);
    }

    return managedThreadId;
  }

  public static StackTraceInfo CreateEventStackTraceInfoOrThrow(
    this TraceEvent @event,
    MutableTraceEventStackSource stackSource)
  {
    var callStackIndex = @event.CallStackIndex();
    if (callStackIndex is CallStackIndex.Invalid) throw new ArgumentOutOfRangeException();

    var currentStackTrace = new List<string>();
    var currentIndex = stackSource.GetCallStack(callStackIndex, @event);
    var managedThreadId = -1;

    while (currentIndex != StackSourceCallStackIndex.Invalid)
    {
      var frame = string.Intern(stackSource.GetFrameName(stackSource.GetFrameIndex(currentIndex), false));
      if (IsThreadStartMethod(frame, out var threadId))
      {
        managedThreadId = threadId;
      }

      currentStackTrace.Add(frame);
      currentIndex = stackSource.GetCallerIndex(currentIndex);
    }

    return new StackTraceInfo((int)callStackIndex, managedThreadId, currentStackTrace.ToArray());
  }
}