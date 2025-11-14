using System.Reflection;
using Microsoft.Diagnostics.Tracing;

namespace Core.Utils;

public static class TraceEventSourceExtensions
{
  private const BindingFlags PrivateInstanceField = BindingFlags.Instance | BindingFlags.NonPublic;


  extension(TraceEventSource log)
  {
    public long GetSyncQpc() =>
      (long)log.GetType().GetField("_syncTimeQPC", PrivateInstanceField)!.GetValue(log)!;

    public long GetQpcFreq() =>
      (long)log.GetType().GetField("_QPCFreq", PrivateInstanceField)!.GetValue(log)!;

    public DateTime GetSyncTimeUtc() =>
      (DateTime)log.GetType().GetField("_syncTimeUTC", PrivateInstanceField)!.GetValue(log)!;
  }
}