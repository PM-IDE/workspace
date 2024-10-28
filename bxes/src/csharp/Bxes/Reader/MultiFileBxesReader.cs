using Bxes.Models.Domain;
using Bxes.Writer;

namespace Bxes.Reader;

public class MultiFileBxesReader : IBxesReader
{
  public EventLogReadResult Read(string path)
  {
    if (!Directory.Exists(path)) throw new SavePathIsNotDirectoryException(path);

    uint? version = null;
    void OpenRead(string fileName, Action<BinaryReader> action)
    {
      using var reader = new BinaryReader(File.OpenRead(Path.Combine(path, fileName)));
      ValidateVersions(ref version, reader.ReadUInt32());
      action(reader);
    }

    var context = new BxesReadContext(null!);

    OpenRead(BxesConstants.SystemMetadataFileName, reader => BxesReadUtils.ReadSystemMetadata(context.WithReader(reader)));
    OpenRead(BxesConstants.ValuesFileName, reader => BxesReadUtils.ReadValues(context.WithReader(reader)));
    OpenRead(BxesConstants.KVPairsFileName, reader => BxesReadUtils.ReadKeyValuePairs(context.WithReader(reader)));

    IEventLogMetadata metadata = null!;
    OpenRead(BxesConstants.MetadataFileName, reader =>
    {
      metadata = BxesReadUtils.ReadMetadata(context.WithReader(reader));
    });

    List<ITraceVariant> variants = null!;
    OpenRead(BxesConstants.TracesFileName, reader =>
    {
      variants = BxesReadUtils.ReadVariants(context.WithReader(reader));
    });

    return new EventLogReadResult(new InMemoryEventLog(version!.Value, metadata, variants), context.SystemMetadata);
  }

  private static void ValidateVersions(ref uint? previousVersion, uint currentVersion)
  {
    if (previousVersion is { } && previousVersion.Value != currentVersion)
    {
      throw new VersionsAreNotEqualException(previousVersion.Value, currentVersion);
    }

    previousVersion = currentVersion;
  }
}

public class VersionsAreNotEqualException(uint firstVersion, uint secondVersion) : BxesException
{
  public override string Message { get; } = $"First version {firstVersion}, is not equal to second one {secondVersion}";
}