namespace Procfiler.Utils;

public static class QpcUtil
{
  //copied and pasted from TraceEventsSource Microsoft.Diagnostics.Tracing.TraceEventSource.QPCTimeToDateTimeUTC
  public static DateTime QpcTimeToDateTimeUtc(long qpcTime, long qpcFreq, long syncTimeQpc, DateTime syncTimeUtc)
  {
    if (qpcTime == long.MaxValue)
      return DateTime.MaxValue;

    var ticks = (long) ((qpcTime - syncTimeQpc) * 10000000.0 / qpcFreq) + syncTimeUtc.Ticks;
    DateTime maxValue;
    if (ticks >= 0L)
    {
      maxValue = DateTime.MaxValue;
      if (maxValue.Ticks >= ticks)
        goto label_5;
    }
    maxValue = DateTime.MaxValue;
    ticks = maxValue.Ticks;

    label_5:
    return new DateTime(ticks, DateTimeKind.Utc);
  }
}