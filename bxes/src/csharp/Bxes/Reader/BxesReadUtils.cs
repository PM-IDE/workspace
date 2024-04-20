using System.IO.Compression;
using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Models.System;
using Bxes.Utils;
using Bxes.Writer;

namespace Bxes.Reader;

public readonly struct ExtractedFileCookie(string filePath) : IDisposable
{
  public FileStream Stream { get; } = File.OpenRead(filePath);


  public void Dispose()
  {
    Stream.Dispose();
    File.Delete(filePath);
  }
}

public static class BxesReadUtils
{
  public static ExtractedFileCookie ReadZipArchive(string path)
  {
    var filePath = Path.GetTempFileName();
    PathUtil.EnsureDeleted(filePath);

    ZipFile.OpenRead(path).Entries.First().ExtractToFile(filePath);
    return new ExtractedFileCookie(filePath);
  }

  public static List<BxesValue> ReadValues(BinaryReader reader)
  {
    var valuesCount = reader.ReadUInt32();
    var values = new List<BxesValue>();

    for (uint i = 0; i < valuesCount; ++i)
    {
      values.Add(BxesValue.Parse(reader, values));
    }

    return values;
  }

  public static List<KeyValuePair<uint, uint>> ReadKeyValuePairs(BinaryReader reader)
  {
    var kvPairsCount = reader.ReadUInt32();
    var keyValues = new List<KeyValuePair<uint, uint>>();

    for (uint i = 0; i < kvPairsCount; ++i)
    {
      var keyIndex = (uint)reader.ReadLeb128Unsigned();
      var valueIndex = (uint)reader.ReadLeb128Unsigned();
      keyValues.Add(new KeyValuePair<uint, uint>(keyIndex, valueIndex));
    }

    return keyValues;
  }

  public static IEventLogMetadata ReadMetadata(
    BinaryReader reader, List<KeyValuePair<uint, uint>> keyValues, List<BxesValue> values)
  {
    var metadata = new EventLogMetadata();

    var propertiesCount = reader.ReadUInt32();
    for (uint i = 0; i < propertiesCount; ++i)
    {
      var kv = keyValues[(int)reader.ReadUInt32()];
      metadata.Properties.Add(new AttributeKeyValue((BxesStringValue)values[(int)kv.Key], values[(int)kv.Value]));
    }

    var extensionsCount = reader.ReadUInt32();
    for (uint i = 0; i < extensionsCount; ++i)
    {
      metadata.Extensions.Add(new BxesExtension
      {
        Name = (BxesStringValue)values[(int)reader.ReadUInt32()],
        Prefix = (BxesStringValue)values[(int)reader.ReadUInt32()],
        Uri = (BxesStringValue)values[(int)reader.ReadUInt32()],
      });
    }

    var globalsEntitiesCount = reader.ReadUInt32();
    for (uint i = 0; i < globalsEntitiesCount; ++i)
    {
      var entityType = (GlobalsEntityKind)reader.ReadByte();
      var globalsCount = reader.ReadUInt32();
      var entityGlobals = new List<AttributeKeyValue>();

      for (uint j = 0; j < globalsCount; ++j)
      {
        var kv = keyValues[(int)reader.ReadUInt32()];
        entityGlobals.Add(new AttributeKeyValue((BxesStringValue)values[(int)kv.Key], values[(int)kv.Value]));
      }

      metadata.Globals.Add(new BxesGlobal
      {
        Kind = entityType,
        Globals = entityGlobals
      });
    }

    var classifiersCount = reader.ReadUInt32();
    for (uint i = 0; i < classifiersCount; ++i)
    {
      var classifierName = (BxesStringValue)values[(int)reader.ReadUInt32()];

      var keys = new List<BxesStringValue>();
      var keysCount = reader.ReadUInt32();
      for (uint j = 0; j < keysCount; ++j)
      {
        keys.Add((BxesStringValue)values[(int)reader.ReadUInt32()]);
      }

      metadata.Classifiers.Add(new BxesClassifier
      {
        Name = classifierName,
        Keys = keys
      });
    }

    return metadata;
  }

  public static List<ValueAttributeDescriptor> ReadValueAttributeDescriptors(BinaryReader reader)
  {
    var valuesAttributesCount = reader.ReadUInt32();
    if (valuesAttributesCount == 0) return [];

    var descriptors = new List<ValueAttributeDescriptor>();
    for (uint i = 0; i < valuesAttributesCount; ++i)
    {
      var typeId = (TypeIds)reader.ReadByte();
      var attributeName = (BxesStringValue)BxesValue.Parse(reader, []);
      
      descriptors.Add(new ValueAttributeDescriptor(typeId, attributeName.Value));
    }

    return descriptors;
  }

  public static List<ITraceVariant> ReadVariants(
    BinaryReader reader, List<KeyValuePair<uint, uint>> keyValues, List<BxesValue> values)
  {
    var variantsCount = reader.ReadUInt32();
    var variants = new List<ITraceVariant>();

    for (uint i = 0; i < variantsCount; ++i)
    {
      var tracesCount = reader.ReadUInt32();

      var metadata = new List<AttributeKeyValue>();
      var metadataCount = reader.ReadUInt32();
      for (uint j = 0; j < metadataCount; ++j)
      {
        var kv = keyValues[(int)reader.ReadUInt32()];
        metadata.Add(new AttributeKeyValue((BxesStringValue)values[(int)kv.Key], values[(int)kv.Value]));
      }

      var eventsCount = reader.ReadUInt32();
      var events = new List<IEvent>();

      for (uint j = 0; j < eventsCount; ++j)
      {
        var name = (BxesStringValue)values[(int)reader.ReadLeb128Unsigned()];
        var timestamp = reader.ReadInt64();

        var attributesCount = reader.ReadLeb128Unsigned();
        var eventAttributes = new List<AttributeKeyValue>();

        for (uint k = 0; k < attributesCount; ++k)
        {
          var kv = keyValues[(int)reader.ReadLeb128Unsigned()];
          eventAttributes.Add(new((BxesStringValue)values[(int)kv.Key], values[(int)kv.Value]));
        }

        events.Add(new InMemoryEventImpl(timestamp, name, eventAttributes));
      }

      variants.Add(new TraceVariantImpl(tracesCount, events, metadata));
    }

    return variants;
  }
}