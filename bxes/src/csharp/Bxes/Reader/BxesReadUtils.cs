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

    using var archive = ZipFile.OpenRead(path);
    archive.Entries.First().ExtractToFile(filePath);

    return new ExtractedFileCookie(filePath);
  }

  public static void ReadValues(BxesReadContext context)
  {
    var valuesCount = context.Reader.ReadUInt32();

    for (uint i = 0; i < valuesCount; ++i)
    {
      context.Metadata.Values.Add(BxesValue.Parse(context.Reader, context.Metadata.Values));
    }
  }

  public static void ReadKeyValuePairs(BxesReadContext context)
  {
    var kvPairsCount = context.Reader.ReadUInt32();

    for (uint i = 0; i < kvPairsCount; ++i)
    {
      var keyIndex = (uint)context.Reader.ReadLeb128Unsigned();
      var valueIndex = (uint)context.Reader.ReadLeb128Unsigned();
      context.Metadata.KeyValues.Add(new KeyValuePair<uint, uint>(keyIndex, valueIndex));
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
      var kv = context.Metadata.KeyValues[(int)context.Reader.ReadUInt32()];
      var attr = new AttributeKeyValue((BxesStringValue)context.Metadata.Values[(int)kv.Key], context.Metadata.Values[(int)kv.Value]);
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
        Name = (BxesStringValue)context.Metadata.Values[(int)context.Reader.ReadUInt32()],
        Prefix = (BxesStringValue)context.Metadata.Values[(int)context.Reader.ReadUInt32()],
        Uri = (BxesStringValue)context.Metadata.Values[(int)context.Reader.ReadUInt32()]
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
        var kv = context.Metadata.KeyValues[(int)context.Reader.ReadUInt32()];
        var key = (BxesStringValue)context.Metadata.Values[(int)kv.Key];
        var value = context.Metadata.Values[(int)kv.Value];

        entityGlobals.Add(new AttributeKeyValue(key, value));
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
      var classifierName = (BxesStringValue)context.Metadata.Values[(int)context.Reader.ReadUInt32()];

      var keys = new List<BxesStringValue>();
      var keysCount = context.Reader.ReadUInt32();
      for (uint j = 0; j < keysCount; ++j)
      {
        keys.Add((BxesStringValue)context.Metadata.Values[(int)context.Reader.ReadUInt32()]);
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
      variants.Add(ReadTraceVariant(context));
    }

    return variants;
  }

  private static TraceVariantImpl ReadTraceVariant(BxesReadContext context)
  {
    var tracesCount = context.Reader.ReadUInt32();

    var metadata = ReadTraceVariantMetadata(context);
    var events = ReadTraceVariantEvents(context);

    return new TraceVariantImpl(tracesCount, events, metadata);
  }

  public static List<IEvent> ReadTraceVariantEvents(BxesReadContext context)
  {
    var eventsCount = context.Reader.ReadUInt32();
    var events = new List<IEvent>((int)eventsCount);

    for (uint j = 0; j < eventsCount; ++j)
    {
      events.Add(ReadEvent(context));
    }

    return events;
  }

  public static List<AttributeKeyValue> ReadTraceVariantMetadata(BxesReadContext context)
  {
    var metadata = new List<AttributeKeyValue>();
    var metadataCount = context.Reader.ReadUInt32();
    for (uint j = 0; j < metadataCount; ++j)
    {
      var kv = context.Metadata.KeyValues[(int)context.Reader.ReadUInt32()];

      metadata.Add(new AttributeKeyValue(
        (BxesStringValue)context.Metadata.Values[(int)kv.Key], context.Metadata.Values[(int)kv.Value]));
    }

    return metadata;
  }

  public static InMemoryEventImpl ReadEvent(BxesReadContext context)
  {
    var name = (BxesStringValue)context.Metadata.Values[(int)context.Reader.ReadLeb128Unsigned()];
    var timestamp = context.Reader.ReadInt64();

    var eventAttributes = new List<AttributeKeyValue>();

    var valueAttributesCount = context.SystemMetadata.ValueAttributeDescriptors.Count;
    for (var i = 0; i < valueAttributesCount; ++i)
    {
      var value = BxesValue.Parse(context.Reader, context.Metadata.Values);
      var (expectedTypeId, valueAttrName) = context.SystemMetadata.ValueAttributeDescriptors[i];

      if (value is not BxesNullValue && value.TypeId != expectedTypeId)
      {
        throw new ValueAttributeTypeNotEqualToDescriptorException(value.TypeId, expectedTypeId);
      }

      if (value is not BxesNullValue || expectedTypeId == TypeIds.Null)
      {
        eventAttributes.Add(new AttributeKeyValue(new BxesStringValue(valueAttrName), value));
      }
    }

    var attributesCount = context.Reader.ReadLeb128Unsigned();

    for (uint k = 0; k < attributesCount; ++k)
    {
      var kv = context.Metadata.KeyValues[(int)context.Reader.ReadLeb128Unsigned()];
      eventAttributes.Add(new AttributeKeyValue((BxesStringValue)context.Metadata.Values[(int)kv.Key],
        context.Metadata.Values[(int)kv.Value]));
    }

    return new InMemoryEventImpl(timestamp, name, eventAttributes);
  }
}

internal class ValueAttributeTypeNotEqualToDescriptorException(TypeIds actual, TypeIds expected) : BxesException
{
  public override string Message { get; } = $"Value attribute type missmatch: expected: {expected}, got {actual}";
}