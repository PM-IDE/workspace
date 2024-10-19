using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;

namespace Bxes.Writer;

public record AttributeKeyValue(BxesStringValue Key, BxesValue Value);

public readonly struct BxesWriteMetadata
{
  public required Dictionary<BxesValue, uint> ValuesIndices { get; init; }
  public required Dictionary<AttributeKeyValue, uint> KeyValueIndices { get; init; }
  public required LogValuesEnumerator ValuesEnumerator { get; init; }
}

public readonly struct BxesWriteContext
{
  public BinaryWriter Writer { get; }
  public BxesWriteMetadata Metadata { get; }


  private BxesWriteContext(
    BinaryWriter writer,
    Dictionary<BxesValue, uint> valuesIndices,
    Dictionary<AttributeKeyValue, uint> keyValueIndices,
    LogValuesEnumerator enumerator)
  {
    Writer = writer;
    Metadata = new BxesWriteMetadata
    {
      ValuesIndices = valuesIndices,
      KeyValueIndices = keyValueIndices,
      ValuesEnumerator = enumerator
    };
  }

  public BxesWriteContext(BinaryWriter binaryWriter, LogValuesEnumerator enumerator)
  {
    Writer = binaryWriter;
    Metadata = new BxesWriteMetadata
    {
      ValuesIndices = [],
      KeyValueIndices = [],
      ValuesEnumerator = enumerator
    };
  }

  public BxesWriteContext(BinaryWriter writer, BxesWriteMetadata metadata)
  {
    Writer = writer;
    Metadata = metadata;
  }

  public BxesWriteContext WithWriter(BinaryWriter writer) => new(writer, Metadata);
}