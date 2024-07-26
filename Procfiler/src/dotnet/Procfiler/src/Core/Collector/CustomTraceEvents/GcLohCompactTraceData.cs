using Core.Constants.TraceEvents;

namespace Procfiler.Core.Collector.CustomTraceEvents;

public class GcLohCompactTraceData(TraceEvent traceEvent)
  : CustomTraceEventBase(traceEvent, TraceEventsConstants.GcLohCompact, ourPayloadEvents)
{
  private static readonly string[] ourPayloadEvents =
  [
    "ClrInstanceID",
    "Count",
    "TimePlan",
    "TimeCompact",
    "TimeRelocate",
    "TotalRefs",
    "ZeroRefs"
  ];


  public int ClrInstanceId => GetInt16At(0);
  public int Count => GetInt16At(2);
  public int TimePlan => GetInt32At(4);
  public int TimeCompact => GetInt32At(8);
  public int TimeRelocate => GetInt32At(12);
  public ulong TotalRefs => GetAddressAt(16);
  public ulong ZeroRefs => GetAddressAt(16 + PointerSize);


  public override object PayloadValue(int index) => index switch
  {
    0 => ClrInstanceId,
    1 => Count,
    2 => TimePlan,
    3 => TimeCompact,
    4 => TimeRelocate,
    5 => TotalRefs,
    6 => ZeroRefs,
    _ => throw new ArgumentOutOfRangeException(nameof(index), index, null)
  };
}