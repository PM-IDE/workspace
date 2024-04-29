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

  public static void ReadValues(BxesReadContext context)
  {
    var valuesCount = context.Reader.ReadUInt32();

    for (uint i = 0; i < valuesCount; ++i)
    {
      context.Values.Add(BxesValue.Parse(context.Reader, context.Values));
    }
  }

  public static void ReadKeyValuePairs(BxesReadContext context)
  {
    var kvPairsCount = context.Reader.ReadUInt32();

    for (uint i = 0; i < kvPairsCount; ++i)
    {
      var keyIndex = (uint)context.Reader.ReadLeb128Unsigned();
      var valueIndex = (uint)context.Reader.ReadLeb128Unsigned();
      context.KeyValues.Add(new KeyValuePair<uint, uint>(keyIndex, valueIndex));
    }
  }

  public static IEventLogMetadata ReadMetadata(BxesReadContext context)
  {
    var metadata = new EventLogMetadata();

    ReadProperties(metadata, context);
    ReadExtensions(metadata, context);
    ReadGlobals(metadata, context);
    ReadClassifiers(metadata, context);

    return metadata;
  }

  private static void ReadProperties(IEventLogMetadata metadata, BxesReadContext context)
  {
    var propertiesCount = context.Reader.ReadUInt32();
    for (uint i = 0; i < propertiesCount; ++i)
    {
      var kv = context.KeyValues[(int)context.Reader.ReadUInt32()];
      var attr = new AttributeKeyValue((BxesStringValue) context.Values[(int) kv.Key], context.Values[(int) kv.Value]);
      metadata.Properties.Add(attr);
    }
  }

  private static void ReadExtensions(IEventLogMetadata metadata, BxesReadContext context)
  {
    var extensionsCount = context.Reader.ReadUInt32();
    for (uint i = 0; i < extensionsCount; ++i)
    {
      metadata.Extensions.Add(new BxesExtension
      {
        Name = (BxesStringValue)context.Values[(int)context.Reader.ReadUInt32()],
        Prefix = (BxesStringValue)context.Values[(int)context.Reader.ReadUInt32()],
        Uri = (BxesStringValue)context.Values[(int)context.Reader.ReadUInt32()],
      });
    }
  }

  private static void ReadGlobals(IEventLogMetadata metadata, BxesReadContext context)
  {
    var globalsEntitiesCount = context.Reader.ReadUInt32();
    for (uint i = 0; i < globalsEntitiesCount; ++i)
    {
      var entityType = (GlobalsEntityKind)context.Reader.ReadByte();
      var globalsCount = context.Reader.ReadUInt32();
      var entityGlobals = new List<AttributeKeyValue>();

      for (uint j = 0; j < globalsCount; ++j)
      {
        var kv = context.KeyValues[(int)context.Reader.ReadUInt32()];
        entityGlobals.Add(new AttributeKeyValue((BxesStringValue)context.Values[(int)kv.Key], context.Values[(int)kv.Value]));
      }

      metadata.Globals.Add(new BxesGlobal
      {
        Kind = entityType,
        Globals = entityGlobals
      });
    }
  }

  private static void ReadClassifiers(IEventLogMetadata metadata, BxesReadContext context)
  {
    var classifiersCount = context.Reader.ReadUInt32();
    for (uint i = 0; i < classifiersCount; ++i)
    {
      var classifierName = (BxesStringValue)context.Values[(int)context.Reader.ReadUInt32()];

      var keys = new List<BxesStringValue>();
      var keysCount = context.Reader.ReadUInt32();
      for (uint j = 0; j < keysCount; ++j)
      {
        keys.Add((BxesStringValue)context.Values[(int)context.Reader.ReadUInt32()]);
      }

      metadata.Classifiers.Add(new BxesClassifier
      {
        Name = classifierName,
        Keys = keys
      });
    }
  }

  public static void ReadSystemMetadata(BxesReadContext context)
  {
    var valuesAttributesCount = context.Reader.ReadUInt32();
    if (valuesAttributesCount == 0) return;

    for (uint i = 0; i < valuesAttributesCount; ++i)
    {
      var typeId = (TypeIds)context.Reader.ReadByte();
      var attributeName = (BxesStringValue)BxesValue.Parse(context.Reader, []);
      
      context.SystemMetadata.ValueAttributeDescriptors.Add(new ValueAttributeDescriptor(typeId, attributeName.Value));
    }
  }

  public static List<ITraceVariant> ReadVariants(BxesReadContext context)
  {
    var variantsCount = context.Reader.ReadUInt32();
    var variants = new List<ITraceVariant>();

    for (uint i = 0; i < variantsCount; ++i)
    {
      var tracesCount = context.Reader.ReadUInt32();

      var metadata = new List<AttributeKeyValue>();
      var metadataCount = context.Reader.ReadUInt32();
      for (uint j = 0; j < metadataCount; ++j)
      {
        var kv = context.KeyValues[(int)context.Reader.ReadUInt32()];
        metadata.Add(new AttributeKeyValue((BxesStringValue)context.Values[(int)kv.Key], context.Values[(int)kv.Value]));
      }

      var eventsCount = context.Reader.ReadUInt32();
      var events = new List<IEvent>();

      for (uint j = 0; j < eventsCount; ++j)
      {
        var name = (BxesStringValue)context.Values[(int)context.Reader.ReadLeb128Unsigned()];
        var timestamp = context.Reader.ReadInt64();

        var attributesCount = context.Reader.ReadLeb128Unsigned();
        var eventAttributes = new List<AttributeKeyValue>();

        for (uint k = 0; k < attributesCount; ++k)
        {
          var kv = context.KeyValues[(int)context.Reader.ReadLeb128Unsigned()];
          eventAttributes.Add(new((BxesStringValue)context.Values[(int)kv.Key], context.Values[(int)kv.Value]));
        }

        events.Add(new InMemoryEventImpl(timestamp, name, eventAttributes));
      }

      variants.Add(new TraceVariantImpl(tracesCount, events, metadata));
    }

    return variants;
  }
}