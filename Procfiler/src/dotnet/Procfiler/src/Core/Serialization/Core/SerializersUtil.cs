using Procfiler.Utils;

namespace Procfiler.Core.Serialization.Core;

public static class SerializersUtil
{
  public const string XesExtension = ".xes";


  public static void DisposeWriters<TWriter>(
    IEnumerable<(string, TWriter)> writers, IProcfilerLogger logger, Action<TWriter> beforeDisposeAction) where TWriter : IDisposable
  {
    using var _ = new PerformanceCookie(nameof(DisposeWriters), logger);

    foreach (var (path, writer) in writers)
    {
      try
      {
        beforeDisposeAction(writer);
        writer.Dispose();
      }
      catch (Exception ex)
      {
        logger.LogWarning(ex, "Failed to dispose writer for path {Path}", path);
      }
    }
  }

  public static void DisposeXesWriters(IEnumerable<(string, XmlWriter)> writers, IProcfilerLogger logger) =>
    DisposeWriters(writers, logger, writer => writer.WriteEndElement());
}