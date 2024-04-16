namespace Procfiler.Utils;

public enum LogFormat
{
  Xes,
  Bxes
}

public static class LogFormatExtensions
{
  public static string GetExtension(this LogFormat format) => format switch
  {
    LogFormat.Bxes => "bxes",
    LogFormat.Xes => "xes",
    _ => throw new ArgumentOutOfRangeException(nameof(format), format, null)
  };
}