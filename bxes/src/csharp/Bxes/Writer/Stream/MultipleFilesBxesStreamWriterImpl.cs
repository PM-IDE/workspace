using Bxes.Models;
using Bxes.Utils;

namespace Bxes.Writer.Stream;

public class MultipleFilesBxesStreamWriterImpl<TEvent> : 
  IBxesStreamWriter, IXesToBxesStatisticsCollector where TEvent : IEvent
{
  private readonly uint myBxesVersion;
  private readonly BinaryWriter myMetadataWriter;
  private readonly BinaryWriter myValuesWriter;
  private readonly BinaryWriter myKeyValuesWriter;
  private readonly BinaryWriter myTracesWriter;
  private readonly IEventLogMetadata myMetadata = new EventLogMetadata();

  private readonly BxesWriteContext myContext = new(null!);
  private readonly ValuesCounter myValuesCounter = new();


  private uint myTracesVariantsCount;

  private uint? myLastTraceVariantEventCount;
  private long myLastTraceVariantCountPosition;


  public MultipleFilesBxesStreamWriterImpl(string savePath, uint bxesVersion)
  {
    if (!Directory.Exists(savePath)) throw new SavePathIsNotDirectoryException(savePath);

    BinaryWriter OpenWrite(string fileName)
    {
      var path = Path.Join(savePath, fileName);
      PathUtil.EnsureDeleted(path);

      return new BinaryWriter(File.OpenWrite(path));
    }

    myBxesVersion = bxesVersion;
    myMetadataWriter = OpenWrite(BxesConstants.MetadataFileName);
    myValuesWriter = OpenWrite(BxesConstants.ValuesFileName);
    myKeyValuesWriter = OpenWrite(BxesConstants.KVPairsFileName);
    myTracesWriter = OpenWrite(BxesConstants.TracesFileName);

    WriteInitialInfo();
  }

  private void WriteInitialInfo()
  {
    myTracesWriter.Write(myBxesVersion);
    myMetadataWriter.Write(myBxesVersion);
    myKeyValuesWriter.Write(myBxesVersion);
    myValuesWriter.Write(myBxesVersion);

    myTracesWriter.Write((uint)0);
    myKeyValuesWriter.Write((uint)0);
    myValuesWriter.Write((uint)0);
  }

  public void HandleEvent(BxesStreamEvent @event)
  {
    switch (@event)
    {
      case BxesRecalculateIndicesEvent:
        RecalculateIndices();
        break;
      case BxesEventEvent<TEvent> eventEvent:
        HandleEventEvent(eventEvent);
        break;
      case BxesKeyValueEvent metadataEvent:
        HandleKeyValueEvent(metadataEvent);
        break;
      case BxesValueEvent valueEvent:
        HandleValueEvent(valueEvent);
        break;
      case BxesLogMetadataGlobalEvent globalEvent:
        myMetadata.Globals.Add(globalEvent.Global);
        break;
      case BxesLogMetadataClassifierEvent classifierEvent:
        myMetadata.Classifiers.Add(classifierEvent.Classifier);
        break;
      case BxesLogMetadataExtensionEvent extensionEvent:
        myMetadata.Extensions.Add(extensionEvent.Extensions);
        break;
      case BxesLogMetadataPropertyEvent propertyEvent:
        myMetadata.Properties.Add(propertyEvent.Attribute);
        break;
      case BxesTraceVariantStartEvent variantStartEvent:
        HandleTraceVariantStart(variantStartEvent);
        break;
      default:
        throw new ArgumentOutOfRangeException(nameof(@event));
    }
  }

  private void RecalculateIndices()
  {
    var valueContext = myContext.WithWriter(myValuesWriter);
    foreach (var value in myValuesCounter.CreateValuesIndices())
    {
      BxesWriteUtils.WriteValueIfNeeded(value, valueContext);
    }

    var kvContext = myContext.WithWriter(myKeyValuesWriter);
    foreach (var kv in myValuesCounter.CreateKeyValueIndices())
    {
      BxesWriteUtils.WriteKeyValuePairIfNeeded(kv, kvContext);
    }
  }

  private void HandleValueEvent(BxesValueEvent valueEvent) => myValuesCounter.HandleValue(valueEvent.Value);
  private void HandleKeyValueEvent(BxesKeyValueEvent @event) => myValuesCounter.HandleKeyValue(@event.MetadataKeyValue);
  
  private void HandleEventEvent(BxesEventEvent<TEvent> @event)
  {
    BxesWriteUtils.WriteEventValues(
      @event.Event, myContext.WithWriter(myValuesWriter), myContext.WithWriter(myKeyValuesWriter));

    BxesWriteUtils.WriteEvent(@event.Event, myContext.WithWriter(myTracesWriter));

    ++myLastTraceVariantEventCount;
  }

  private void HandleTraceVariantStart(BxesTraceVariantStartEvent @event)
  {
    WriteLastTraceVariantCountIfNeeded();

    myLastTraceVariantEventCount = 0;
    myTracesWriter.Write(@event.TracesCount);

    foreach (var pair in @event.Metadata)
    {
      BxesWriteUtils.WriteValueIfNeeded(pair.Key, myContext.WithWriter(myValuesWriter));
      BxesWriteUtils.WriteValueIfNeeded(pair.Value, myContext.WithWriter(myValuesWriter));

      BxesWriteUtils.WriteKeyValuePairIfNeeded(pair, myContext.WithWriter(myKeyValuesWriter));
    }

    BxesWriteUtils.WriteVariantMetadata(@event.Metadata, myContext.WithWriter(myTracesWriter));

    myLastTraceVariantCountPosition = myTracesWriter.BaseStream.Position;
    myTracesWriter.Write((uint)0);

    ++myTracesVariantsCount;
  }

  private void WriteLastTraceVariantCountIfNeeded()
  {
    if (myLastTraceVariantEventCount is null) return;

    BxesWriteUtils.WriteCount(myTracesWriter, myLastTraceVariantCountPosition, myLastTraceVariantEventCount.Value);
    myLastTraceVariantEventCount = null;
  }

  public void Dispose()
  {
    FlushInformation();

    myMetadataWriter.Dispose();
    myKeyValuesWriter.Dispose();
    myValuesWriter.Dispose();
    myTracesWriter.Dispose();
  }

  private void FlushInformation()
  {
    WriteMetadata();
    WriteLastTraceVariantCountIfNeeded();

    const int CountPos = sizeof(uint);

    BxesWriteUtils.WriteCount(myTracesWriter, CountPos, myTracesVariantsCount);
    BxesWriteUtils.WriteCount(myValuesWriter, CountPos, (uint)myContext.ValuesIndices.Count);
    BxesWriteUtils.WriteCount(myKeyValuesWriter, CountPos, (uint)myContext.KeyValueIndices.Count);
  }

  private void WriteMetadata()
  {
    foreach (var value in myMetadata.EnumerateValues())
    {
      BxesWriteUtils.WriteValueIfNeeded(value, myContext.WithWriter(myValuesWriter));
    }

    foreach (var kv in myMetadata.EnumerateKeyValuePairs())
    {
      BxesWriteUtils.WriteKeyValuePairIfNeeded(kv, myContext.WithWriter(myKeyValuesWriter));
    }

    BxesWriteUtils.WriteEventLogMetadata(myMetadata, myContext.WithWriter(myMetadataWriter));
  }

  public XesToBxesConversionStatistics ObtainStatistics() => myValuesCounter.ToStatistics();
}