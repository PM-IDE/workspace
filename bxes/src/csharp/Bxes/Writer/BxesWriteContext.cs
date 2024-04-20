using Bxes.Models;
using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;

namespace Bxes.Writer;

public record AttributeKeyValue(BxesStringValue Key, BxesValue Value);

public readonly struct BxesWriteContext
{
  public BinaryWriter Writer { get; }
  public Dictionary<BxesValue, uint> ValuesIndices { get; }
  public Dictionary<AttributeKeyValue, uint> KeyValueIndices { get; }


  private BxesWriteContext(
    BinaryWriter writer,
    Dictionary<BxesValue, uint> valuesIndices,
    Dictionary<AttributeKeyValue, uint> keyValueIndices)
  {
    Writer = writer;
    ValuesIndices = valuesIndices;
    KeyValueIndices = keyValueIndices;
  }

  public BxesWriteContext(BinaryWriter binaryWriter)
  {
    Writer = binaryWriter;
    ValuesIndices = new Dictionary<BxesValue, uint>();
    KeyValueIndices = new Dictionary<AttributeKeyValue, uint>();
  }


  public BxesWriteContext WithWriter(BinaryWriter writer) => new(writer, ValuesIndices, KeyValueIndices);
}