using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;

namespace Bxes.Writer;

public record AttributeKeyValue(BxesStringValue Key, BxesValue Value);

public readonly struct BxesWriteMetadata
{
  public required Dictionary<BxesValue, uint> ValuesIndices { get; init; }
  public required Dictionary<AttributeKeyValue, uint> KeyValueIndices { get; init; }
}

public readonly struct BxesWriteContext
{
  public BinaryWriter Writer { get; }
  public BxesWriteMetadata Metadata { get; }
  public LogValuesEnumerator ValuesEnumerator { get; }


  private BxesWriteContext(
    BinaryWriter writer,
    Dictionary<BxesValue, uint> valuesIndices,
    Dictionary<AttributeKeyValue, uint> keyValueIndices,
    LogValuesEnumerator enumerator)
  {
    Writer = writer;
    ValuesEnumerator = enumerator;
    Metadata = new BxesWriteMetadata
    {
      ValuesIndices = valuesIndices,
      KeyValueIndices = keyValueIndices
    };
  }

  public BxesWriteContext(BinaryWriter binaryWriter, LogValuesEnumerator enumerator)
  {
    Writer = binaryWriter;
    ValuesEnumerator = enumerator;
    Metadata = new BxesWriteMetadata
    {
      ValuesIndices = [],
      KeyValueIndices = []
    };
  }


  public BxesWriteContext WithWriter(BinaryWriter writer) => new(writer, Metadata.ValuesIndices, Metadata.KeyValueIndices, ValuesEnumerator);
}