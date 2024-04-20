using Bxes.Models.Domain;

namespace Bxes.Reader;

public class SingleFileBxesReader : IBxesReader
{
  public EventLogReadResult Read(string path)
  {
    using var cookie = BxesReadUtils.ReadZipArchive(path);
    using var br = new BinaryReader(cookie.Stream);

    var version = br.ReadUInt32();
    var systemMetadata = BxesReadUtils.ReadSystemMetadata(br);
    var values = BxesReadUtils.ReadValues(br);
    var keyValues = BxesReadUtils.ReadKeyValuePairs(br);
    var metadata = BxesReadUtils.ReadMetadata(br, keyValues, values);
    var variants = BxesReadUtils.ReadVariants(br, keyValues, values);

    return new EventLogReadResult(new InMemoryEventLog(version, metadata, variants), systemMetadata);
  }
}