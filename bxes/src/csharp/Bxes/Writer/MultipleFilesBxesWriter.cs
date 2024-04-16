using Bxes.Models;
using Bxes.Utils;

namespace Bxes.Writer;

public class MultipleFilesBxesWriter : IBxesWriter
{
  public void Write(IEventLog log, string savePath)
  {
    if (!Directory.Exists(savePath))
    {
      throw new SavePathIsNotDirectoryException(savePath);
    }

    var context = new BxesWriteContext(null!);

    void Write(BinaryWriter writer, Action<IEventLog, BxesWriteContext> writeAction) =>
      writeAction(log, context.WithWriter(writer));

    var version = log.Version;
    ExecuteWithFile(savePath, BxesConstants.ValuesFileName, version, bw => Write(bw, BxesWriteUtils.WriteValues));
    ExecuteWithFile(savePath, BxesConstants.KVPairsFileName, version,
      bw => Write(bw, BxesWriteUtils.WriteKeyValuePairs));

    ExecuteWithFile(
      savePath,
      BxesConstants.MetadataFileName,
      version, bw => Write(bw, (log, context) => BxesWriteUtils.WriteEventLogMetadata(log.Metadata, context)));

    ExecuteWithFile(savePath, BxesConstants.TracesFileName, version,
      bw => Write(bw, BxesWriteUtils.WriteTracesVariants));
  }

  private static void ExecuteWithFile(
    string saveDirectory, string fileName, uint version, Action<BinaryWriter> writeAction)
  {
    var path = Path.Combine(saveDirectory, fileName);
    PathUtil.EnsureDeleted(path);

    BxesWriteUtils.ExecuteWithFile(path, writer =>
    {
      BxesWriteUtils.WriteBxesVersion(writer, version);
      writeAction(writer);
    });
  }
}

public class SavePathIsNotDirectoryException(string savePath) : BxesException
{
  public override string Message { get; } = $"The {savePath} is not a directory or it does not exist";
}