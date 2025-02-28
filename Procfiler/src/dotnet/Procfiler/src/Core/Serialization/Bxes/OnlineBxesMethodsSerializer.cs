using System.Collections.Immutable;
using Bxes.Writer;
using Bxes.Writer.Stream;
using Core.Bxes;
using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Core.EventRecord;
using Procfiler.Core.Serialization.Core;
using Procfiler.Core.SplitByMethod;

namespace Procfiler.Core.Serialization.Bxes;

public class BxesWriteStateWithLastEvent : BxesWriteState
{
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
      Writer = new SingleFileBxesStreamWriterImpl<BxesEvent>(filePath, 1, BxesUtil.CreateSystemMetadata())
    });
  }

  protected override void HandleUpdate(EventUpdateBase<BxesWriteStateWithLastEvent> update)
  {
    if (update.FrameInfo.State is null) return;

    switch (update)
    {
      case MethodExecutionUpdate<BxesWriteStateWithLastEvent> methodExecutionUpdate:
        var state = update.FrameInfo.State;
        var executionEvent = CurrentFrameInfoUtil.CreateMethodExecutionEvent(
          methodExecutionUpdate.FrameInfo,
          Factory,
          methodExecutionUpdate.MethodName,
          update.FrameInfo.State!.LastWrittenEvent
        );

        WriteEvent(state, executionEvent);
        break;
      case MethodFinishedUpdate<BxesWriteStateWithLastEvent>:
        break;
      case MethodStartedUpdate<BxesWriteStateWithLastEvent>:
        update.FrameInfo.State.Writer.HandleEvent(new BxesTraceVariantStartEvent(1, ImmutableList<AttributeKeyValue>.Empty));
        break;
      case NormalEventUpdate<BxesWriteStateWithLastEvent> normalEventUpdate:
        WriteEvent(update.FrameInfo.State, normalEventUpdate.Event);
        break;
      default:
        throw new ArgumentOutOfRangeException(nameof(update));
    }
  }

  private void WriteEvent(BxesWriteStateWithLastEvent state, EventRecordWithMetadata eventRecord)
  {
    state.LastWrittenEvent = eventRecord;
    state.Writer.HandleEvent(new BxesEventEvent<BxesEvent>(new BxesEvent(eventRecord, WriteAllEventMetadata)));
  }

  public override void Dispose()
  {
    SerializersUtil.DisposeWriters(States.Select(pair => (pair.Key, pair.Value.Writer)), Logger, _ => { });
  }
}