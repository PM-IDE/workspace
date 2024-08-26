using Bxes.Models.Domain;
using Bxes.Models.System;

namespace Bxes.Reader;

public struct BxesReadMetadata
{
  public required List<BxesValue> Values { get; init; }
  public required List<KeyValuePair<uint, uint>> KeyValues { get; init; }
}

public readonly struct BxesReadContext(
  BinaryReader reader,
  BxesReadMetadata readMetadata,
  ISystemMetadata metadata)
{
  public BinaryReader Reader { get; } = reader;
  public BxesReadMetadata Metadata { get; } = readMetadata;
  public ISystemMetadata SystemMetadata { get; } = metadata;


  public BxesReadContext(BinaryReader reader) : this(reader, new BxesReadMetadata { Values = [], KeyValues = [] }, new SystemMetadata())
  {
  }


  public BxesReadContext WithReader(BinaryReader reader) => new(reader, Metadata, SystemMetadata);
}