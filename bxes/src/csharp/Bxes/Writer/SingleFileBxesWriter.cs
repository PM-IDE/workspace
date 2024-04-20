using Bxes.Models;
using Bxes.Utils;

namespace Bxes.Writer;

public class SingleFileBxesWriter(List<ValueAttributeDescriptor> descriptors) : IBxesWriter
{
  public void Write(IEventLog log, string savePath)
  {
    PathUtil.EnsureDeleted(savePath);

    using var cookie = new TempFilePathContainer();
    BxesWriteUtils.ExecuteWithFile(cookie.Path, writer =>
    {
      var context = new BxesWriteContext(writer);

      BxesWriteUtils.WriteBxesVersion(writer, log.Version);
      BxesWriteUtils.WriteValuesAttributesDescriptors(descriptors, context);
      BxesWriteUtils.WriteValues(log, context);
      BxesWriteUtils.WriteKeyValuePairs(log, context);
      BxesWriteUtils.WriteEventLogMetadata(log.Metadata, context);
      BxesWriteUtils.WriteTracesVariants(log, context);
    });

    BxesWriteUtils.CreateZipArchive(new[] { cookie.Path }, savePath);
  }
}