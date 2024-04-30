using Bxes.Models.Domain;
using Bxes.Models.System;
using Bxes.Utils;

namespace Bxes.Writer;

public class SingleFileBxesWriter(ISystemMetadata metadata) : IBxesWriter
{
  public void Write(IEventLog log, string savePath)
  {
    PathUtil.EnsureDeleted(savePath);

    using var cookie = new TempFilePathContainer();
    BxesWriteUtils.ExecuteWithFile(cookie.Path, writer =>
    {
      var context = new BxesWriteContext(writer, new LogValuesEnumerator(metadata.ValueAttributeDescriptors));

      BxesWriteUtils.WriteBxesVersion(writer, log.Version);
      BxesWriteUtils.WriteValuesAttributesDescriptors(context.ValuesEnumerator.OrderedValueAttributes, context);
      BxesWriteUtils.WriteValues(log, context);
      BxesWriteUtils.WriteKeyValuePairs(log, context);
      BxesWriteUtils.WriteEventLogMetadata(log.Metadata, context);
      BxesWriteUtils.WriteTracesVariants(log, context);
    });

    BxesWriteUtils.CreateZipArchive(new[] { cookie.Path }, savePath);
  }
}