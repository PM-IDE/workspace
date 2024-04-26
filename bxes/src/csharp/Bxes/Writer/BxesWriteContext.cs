using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;

namespace Bxes.Writer;

public record AttributeKeyValue(BxesStringValue Key, BxesValue Value);

public readonly struct BxesWriteContext
{
  public BinaryWriter Writer { get; }
  public Dictionary<BxesValue, uint> ValuesIndices { get; }
  public Dictionary<AttributeKeyValue, uint> KeyValueIndices { get; }
  public LogValuesEnumerator ValuesEnumerator { get; }


  private BxesWriteContext(
    BinaryWriter writer,
    Dictionary<BxesValue, uint> valuesIndices,
    Dictionary<AttributeKeyValue, uint> keyValueIndices,
    LogValuesEnumerator enumerator)
  {
    Writer = writer;
    ValuesIndices = valuesIndices;
    KeyValueIndices = keyValueIndices;
    ValuesEnumerator = enumerator;
  }

  public BxesWriteContext(BinaryWriter binaryWriter, LogValuesEnumerator enumerator)
  {
    Writer = binaryWriter;
    ValuesIndices = new Dictionary<BxesValue, uint>();
    KeyValueIndices = new Dictionary<AttributeKeyValue, uint>();
    ValuesEnumerator = enumerator;
  }


  public BxesWriteContext WithWriter(BinaryWriter writer) => new(writer, ValuesIndices, KeyValueIndices, ValuesEnumerator);
}