using System.Reflection;
using Microsoft.Diagnostics.Tracing;

namespace Core.Utils;

public static class TraceEventSourceExtensions
{
  private const BindingFlags PrivateInstanceField = BindingFlags.Instance | BindingFlags.NonPublic;


  public static long GetSyncQpc(this TraceEventSource log)
  {
    return (long)log.GetType().GetField("_syncTimeQPC", PrivateInstanceField)!.GetValue(log)!;
  }

  public static long GetQpcFreq(this TraceEventSource log)
  {
    return (long)log.GetType().GetField("_QPCFreq", PrivateInstanceField)!.GetValue(log)!;
  }

  public static DateTime GetSyncTimeUtc(this TraceEventSource log)
  {
    return (DateTime)log.GetType().GetField("_syncTimeUTC", PrivateInstanceField)!.GetValue(log)!;
  }
}