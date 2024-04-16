using System.Text;

namespace Bxes;

public static class BxesConstants
{
  public const uint BxesVersion = 1;

  public const string MetadataFileName = "metadata.bxes";
  public const string ValuesFileName = "values.bxes";
  public const string KVPairsFileName = "kvpair.bxes";
  public const string TracesFileName = "traces.bxes";

  public static Encoding BxesEncoding { get; } = Encoding.UTF8;
}