using Bxes.Models;
using Bxes.Models.System;
using Bxes.Writer;

namespace Bxes.Reader;

public class MultiFileBxesReader : IBxesReader
{
  public IEventLog Read(string path)
  {
    if (!Directory.Exists(path)) throw new SavePathIsNotDirectoryException(path);

    void OpenRead(string fileName, Action<BinaryReader> action)
    {
      using var reader = new BinaryReader(File.OpenRead(Path.Combine(path, fileName)));
      action(reader);
    }

    uint? version = null;

    List<ValueAttributeDescriptor> descriptors = null!;
    OpenRead(BxesConstants.SystemMetadataFileName, reader =>
    {
      ValidateVersions(ref version, reader.ReadUInt32());
      descriptors = BxesReadUtils.ReadValueAttributeDescriptors(reader);
    });

    List<BxesValue> values = null!;
    OpenRead(BxesConstants.ValuesFileName, reader =>
    {
      ValidateVersions(ref version, reader.ReadUInt32());
      values = BxesReadUtils.ReadValues(reader);
    });

    List<KeyValuePair<uint, uint>> keyValues = null!;
    OpenRead(BxesConstants.KVPairsFileName, reader =>
    {
      ValidateVersions(ref version, reader.ReadUInt32());
      keyValues = BxesReadUtils.ReadKeyValuePairs(reader);
    });

    IEventLogMetadata metadata = null!;
    OpenRead(BxesConstants.MetadataFileName, reader =>
    {
      ValidateVersions(ref version, reader.ReadUInt32());
      metadata = BxesReadUtils.ReadMetadata(reader, keyValues, values);
    });

    List<ITraceVariant> variants = null!;
    OpenRead(BxesConstants.TracesFileName, reader =>
    {
      ValidateVersions(ref version, reader.ReadUInt32());
      variants = BxesReadUtils.ReadVariants(reader, keyValues, values);
    });

    return new InMemoryEventLog(version!.Value, metadata, variants);
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