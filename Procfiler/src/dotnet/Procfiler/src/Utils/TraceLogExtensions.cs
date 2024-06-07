namespace Procfiler.Utils;

public static class TraceLogExtensions
{
  private const BindingFlags PrivateInstanceField = BindingFlags.Instance | BindingFlags.NonPublic;


  public static long GetSyncQpc(this TraceLog log)
  {
    return (long)log.GetType().GetField("_syncTimeQPC", PrivateInstanceField)!.GetValue(log)!;
  }

  public static long GetQpcFreq(this TraceLog log)
  {
    return (long)log.GetType().GetField("_QPCFreq", PrivateInstanceField)!.GetValue(log)!;
  }

  public static DateTime GetSyncTimeUtc(this TraceLog log)
  {
    return (DateTime)log.GetType().GetField("_syncTimeUTC", PrivateInstanceField)!.GetValue(log)!;
  }
}