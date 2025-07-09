using System.Collections.Immutable;
using Bxes.Writer;
using Bxes.Writer.Stream;
using Core.Bxes;
using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Core.EventRecord;
using Procfiler.Core.Serialization.Core;
using Procfiler.Core.SplitByMethod;
using ProcfilerLoggerProvider;

namespace Procfiler.Core.Serialization.Bxes;

public class BxesWriteStateWithLastEvent : BxesWriteState
{
  public required string FileName { get; init; }
  public EventRecordWithMetadata? LastWrittenEvent { get; set; }
}

public class OnlineBxesMethodsSerializer(
  string outputDirectory,
  Regex? targetMethodsRegex,
  IFullMethodNameBeautifier methodNameBeautifier,
  IProcfilerEventsFactory factory,
  IProcfilerLogger logger,
  bool writeAllEventMetadata)
  : OnlineMethodsSerializerBase<BxesWriteStateWithLastEvent>(
    outputDirectory, targetMethodsRegex, methodNameBeautifier, factory, logger, writeAllEventMetadata)
{
  private const string BxesExtension = ".bxes";

  protected override BxesWriteStateWithLastEvent? TryCreateStateInternal(EventRecordWithMetadata contextEvent)
  {
    var methodName = contextEvent.GetMethodStartEndEventInfo().Frame;
    var name = FullMethodNameBeautifier.Beautify(methodName);
    if (!name.EndsWith(BxesExtension))
    {
      name += BxesExtension;
    }

    var filePath = Path.Join(OutputDirectory, name);

    return States.GetOrCreate(filePath, () => new BxesWriteStateWithLastEvent
    {
      FileName = Path.GetFileNameWithoutExtension(filePath),
      Writer = new SingleFileBxesStreamWriterImpl<BxesEvent>(filePath, 1, BxesUtil.CreateSystemMetadata())
    });
  }

  public override void HandleUpdate(EventUpdateBase update)
  {
    if (update.FrameInfo.State is not BxesWriteStateWithLastEvent state) return;

    switch (update)
    {
      case MethodExecutionUpdate methodExecutionUpdate:
        var executionEvent = CurrentFrameInfoUtil.CreateMethodExecutionEvent(
          methodExecutionUpdate.FrameInfo.Frame,
          Factory,
          methodExecutionUpdate.MethodName,
          state.LastWrittenEvent
        );

        WriteEvent(state, executionEvent);
        break;
      case MethodFinishedUpdate:
        break;
      case MethodStartedUpdate:
        state.Writer.HandleEvent(new BxesTraceVariantStartEvent(1, ImmutableList<AttributeKeyValue>.Empty));
        break;
      case NormalEventUpdate normalEventUpdate:
        WriteEvent(state, normalEventUpdate.Event);
        break;
      default:
        throw new ArgumentOutOfRangeException(nameof(update));
    }
  }

  private void WriteEvent(BxesWriteStateWithLastEvent state, EventRecordWithMetadata eventRecord)
  {
    OcelLogger.LogGloballyAttachedObject(eventRecord, $"BXES_{state.FileName}", eventRecord.EventClass);

    state.LastWrittenEvent = eventRecord;
    state.Writer.HandleEvent(new BxesEventEvent<BxesEvent>(new BxesEvent(eventRecord, WriteAllEventMetadata)));
  }

  public override void Dispose()
  {
    SerializersUtil.DisposeWriters(States.Select(pair => (pair.Key, pair.Value.Writer)), Logger, _ => { });
  }
}