namespace Procfiler.Utils;

public enum FileFormat
{
  Csv,
  MethodCallTree,
}

public static class FileFormatExtensions
{
  public static string GetExtension(this FileFormat format) => format switch
  {
    FileFormat.Csv => "csv",
    FileFormat.MethodCallTree => "mtree",
    _ => throw new ArgumentOutOfRangeException(nameof(format), format, null)
  };
}