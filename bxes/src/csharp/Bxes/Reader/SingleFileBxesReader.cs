using Bxes.Models.Domain;

namespace Bxes.Reader;

public class SingleFileBxesReader : IBxesReader
{
  public EventLogReadResult Read(string path)
  {
    using var cookie = BxesReadUtils.ReadZipArchive(path);
    using var br = new BinaryReader(cookie.Stream);

    var context = new BxesReadContext(br);
    var version = br.ReadUInt32();
    var systemMetadata = BxesReadUtils.ReadSystemMetadata(context);
    BxesReadUtils.ReadValues(context);
    BxesReadUtils.ReadKeyValuePairs(context);
    var metadata = BxesReadUtils.ReadMetadata(context);
    var variants = BxesReadUtils.ReadVariants(context);

    return new EventLogReadResult(new InMemoryEventLog(version, metadata, variants), systemMetadata);
  }
}