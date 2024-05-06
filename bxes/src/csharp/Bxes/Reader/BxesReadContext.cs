using Bxes.Models.Domain;
using Bxes.Models.System;

namespace Bxes.Reader;

public readonly struct BxesReadContext(
  BinaryReader reader,
  List<BxesValue> values,
  List<KeyValuePair<uint, uint>> keyValues,
  ISystemMetadata metadata)
{
  public BinaryReader Reader { get; } = reader;
  public List<BxesValue> Values { get; } = values;
  public List<KeyValuePair<uint, uint>> KeyValues { get; } = keyValues;
  public ISystemMetadata SystemMetadata { get; } = metadata;


  public BxesReadContext(BinaryReader reader) : this(reader, [], [], new SystemMetadata())
  {
  }


  public BxesReadContext WithReader(BinaryReader reader) => new(reader, Values, KeyValues, SystemMetadata);
}