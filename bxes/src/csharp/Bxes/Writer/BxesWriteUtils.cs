using System.IO.Compression;
using Bxes.Models.Domain;
using Bxes.Models.Domain.Values;
using Bxes.Models.System;
using Bxes.Utils;

namespace Bxes.Writer;

using IndexType = uint;

internal static class BxesWriteUtils
{
  private static void WriteCollectionAndCount<TElement>(
    IEnumerable<TElement> collection,
    BxesWriteContext context,
    Action<TElement, BxesWriteContext> elementWriter,
    Func<IndexType> countGetter)
  {
    var countPos = context.Writer.BaseStream.Position;
    context.Writer.Write((IndexType)0);

    foreach (var element in collection)
    {
      elementWriter(element, context);
    }

    WriteCount(context.Writer, countPos, countGetter());
  }

  private static void WriteCollection<TElement>(
    IEnumerable<TElement> collection,
    int count,
    BxesWriteContext context,
    bool writeLeb128Count,
    Action<TElement, BxesWriteContext> elementWriter)
  {
    if (writeLeb128Count)
    {
      context.Writer.WriteLeb128Unsigned((IndexType)count);
    }
    else
    {
      context.Writer.Write((IndexType)count);
    }

    foreach (var element in collection)
    {
      elementWriter(element, context);
    }
  }

  public static void WriteCount(BinaryWriter writer, long countPos, IndexType count)
  {
    var currentPosition = writer.BaseStream.Position;

    writer.BaseStream.Seek(countPos, SeekOrigin.Begin);
    writer.Write(count);

    writer.BaseStream.Seek(currentPosition, SeekOrigin.Begin);
  }

  public static void WriteBxesVersion(BinaryWriter writer, IndexType version) => writer.Write(version);

  public static void WriteEventValues<TEvent>(
    TEvent @event, BxesWriteContext valuesContext, BxesWriteContext keyValuesContext) where TEvent : IEvent
  {
    WriteValueIfNeeded(new BxesStringValue(@event.Name), valuesContext);

    foreach (var value in valuesContext.ValuesEnumerator.EnumerateEventValues(@event))
    {
      WriteValueIfNeeded(value, valuesContext);
    }

    foreach (var keyValue in valuesContext.ValuesEnumerator.EnumerateEventKeyValuePairs(@event))
    {
      if (keyValuesContext.KeyValueIndices.ContainsKey(keyValue)) continue;

      WriteKeyValuePair(keyValue, keyValuesContext);
    }
  }

  public static void WriteValueIfNeeded(BxesValue value, BxesWriteContext context)
  {
    if (context.ValuesIndices.ContainsKey(value)) return;

    value.WriteTo(context);

    context.ValuesIndices[value] = (IndexType)context.ValuesIndices.Count;
  }

  public static void WriteKeyValuePairs(IEventLog log, BxesWriteContext context)
  {
    var pairs = context.ValuesEnumerator.EnumerateKeyValues(log);
    WriteCollectionAndCount(pairs, context, WriteKeyValuePairIfNeeded, () => (IndexType)context.KeyValueIndices.Count);
  }

  public static void WriteKeyValuePairIfNeeded(AttributeKeyValue pair, BxesWriteContext context)
  {
    if (context.KeyValueIndices.ContainsKey(pair)) return;

    WriteKeyValuePair(pair, context);
  }

  private static void WriteKeyValuePair(AttributeKeyValue pair, BxesWriteContext context)
  {
    context.Writer.WriteLeb128Unsigned(context.ValuesIndices[pair.Key]);
    context.Writer.WriteLeb128Unsigned(context.ValuesIndices[pair.Value]);

    context.KeyValueIndices[pair] = (IndexType)context.KeyValueIndices.Count;
  }

  public static void WriteEventLogMetadata(IEventLogMetadata metadata, BxesWriteContext context)
  {
    WriteProperties(metadata.Properties, context);
    WriteExtensions(metadata.Extensions, context);
    WriteGlobals(metadata.Globals, context);
    WriteClassifiers(metadata.Classifiers, context);
  }

  private static void WriteProperties(IList<AttributeKeyValue> properties, BxesWriteContext context)
  {
    context.Writer.Write((IndexType)properties.Count);
    foreach (var pair in properties)
    {
      context.Writer.Write(context.KeyValueIndices[pair]);
    }
  }

  private static void WriteExtensions(IList<BxesExtension> extensions, BxesWriteContext context)
  {
    context.Writer.Write((IndexType)extensions.Count);
    foreach (var extension in extensions)
    {
      context.Writer.Write(context.ValuesIndices[extension.Name]);
      context.Writer.Write(context.ValuesIndices[extension.Prefix]);
      context.Writer.Write(context.ValuesIndices[extension.Uri]);
    }
  }

  private static void WriteGlobals(IList<BxesGlobal> globals, BxesWriteContext context)
  {
    context.Writer.Write((IndexType)globals.Count);
    foreach (var entityGlobal in globals)
    {
      context.Writer.Write((byte)entityGlobal.Kind);
      context.Writer.Write((IndexType)entityGlobal.Globals.Count);

      foreach (var global in entityGlobal.Globals)
      {
        context.Writer.Write(context.KeyValueIndices[global]);
      }
    }
  }

  private static void WriteClassifiers(IList<BxesClassifier> classifiers, BxesWriteContext context)
  {
    context.Writer.Write((IndexType)classifiers.Count);
    foreach (var classifier in classifiers)
    {
      context.Writer.Write(context.ValuesIndices[classifier.Name]);
      context.Writer.Write((IndexType)classifier.Keys.Count);

      foreach (var key in classifier.Keys)
      {
        context.Writer.Write(context.ValuesIndices[new BxesStringValue(key.Value)]);
      }
    }
  }

  public static void WriteValuesAttributesDescriptors(
    IReadOnlyList<ValueAttributeDescriptor> descriptors, BxesWriteContext context)
  {
    context.Writer.Write((IndexType)descriptors.Count);
    foreach (var (typeId, name) in descriptors)
    {
      context.Writer.Write((byte)typeId);
      new BxesStringValue(name).WriteTo(context);
    }
  }

  private static void WriteKeyValueIndex(AttributeKeyValue tuple, BxesWriteContext context)
  {
    context.Writer.WriteLeb128Unsigned(context.KeyValueIndices[tuple]);
  }

  public static void WriteTracesVariants(IEventLog log, BxesWriteContext context) =>
    WriteCollectionAndCount(log.Traces, context, WriteTraceVariant, () => (IndexType)log.Traces.Count);

  private static void WriteTraceVariant(ITraceVariant variant, BxesWriteContext context)
  {
    context.Writer.Write(variant.Count);

    WriteVariantMetadata(variant.Metadata, context);
    WriteCollectionAndCount(variant.Events, context, WriteEvent, () => (IndexType)variant.Events.Count);
  }

  public static void WriteVariantMetadata(IList<AttributeKeyValue> metadata, BxesWriteContext context)
  {
    var metadataCount = (IndexType)metadata.Count;
    context.Writer.Write(metadataCount);
    foreach (var pair in metadata)
    {
      context.Writer.Write(context.KeyValueIndices[pair]);
    }
  }

  public static void WriteEvent(IEvent @event, BxesWriteContext context)
  {
    context.Writer.WriteLeb128Unsigned(context.ValuesIndices[new BxesStringValue(@event.Name)]);
    context.Writer.Write(@event.Timestamp);

    var (valueAttrs, defaultAttrs, defaultAttrsCount) = context.ValuesEnumerator.SplitEventAttributesOrThrow(@event);
    if (valueAttrs.Count != 0)
    {
      WriteEventValueAttributes(valueAttrs, context);
    }

    WriteCollection(defaultAttrs, defaultAttrsCount, context, true, WriteKeyValueIndex);
  }

  private static void WriteEventValueAttributes(
    IEnumerable<AttributeKeyValue> valueAttributes, BxesWriteContext context)
  {
    foreach (var (_, value) in valueAttributes)
    {
      value.WriteTo(context);
    }
  }

  public static void WriteValues(IEventLog log, BxesWriteContext context)
  {
    var values = context.ValuesEnumerator.EnumerateValues(log);
    WriteCollectionAndCount(values, context, WriteValueIfNeeded, () => (IndexType)context.ValuesIndices.Count);
  }

  public static void ExecuteWithFile(string filePath, Action<BinaryWriter> writeAction)
  {
    using var fs = new FileStream(filePath, new FileStreamOptions
    {
      Access = FileAccess.Write,
      Mode = FileMode.Create,
      Options = FileOptions.RandomAccess,
      BufferSize = 1024 * 16
    });

    using var bw = new BinaryWriter(fs, BxesConstants.BxesEncoding);

    writeAction(bw);
  }

  public static void CreateZipArchive(IEnumerable<string> filesPaths, string outputPath)
  {
    using var fs = File.OpenWrite(outputPath);
    using var archive = new ZipArchive(fs, ZipArchiveMode.Create);

    foreach (var filePath in filesPaths)
    {
      var fileName = Path.GetFileName(filePath);
      archive.CreateEntryFromFile(filePath, fileName, CompressionLevel.SmallestSize);
    }
  }
}