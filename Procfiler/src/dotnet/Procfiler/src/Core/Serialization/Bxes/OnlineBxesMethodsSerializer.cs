using System.Collections.Immutable;
using System.Collections.ObjectModel;
using Bxes.Models;
using Bxes.Models.Values;
using Bxes.Models.Values.Lifecycle;
using Bxes.Writer;
using Bxes.Writer.Stream;
using Procfiler.Core.EventRecord;
using Procfiler.Core.Serialization.Core;
using Procfiler.Core.SplitByMethod;
using Procfiler.Utils;

namespace Procfiler.Core.Serialization.Bxes;

public class BxesEvent : IEvent
{
  public long Timestamp { get; }
  public string Name { get; }
  public IEventLifecycle Lifecycle { get; }
  public IList<AttributeKeyValue> Attributes { get; }


  public BxesEvent(EventRecordWithMetadata eventRecord, bool writeAllEventMetadata)
  {
    Timestamp = eventRecord.Stamp;
    Name = eventRecord.EventName;
    Lifecycle = new BrafLifecycle(BrafLifecycleValues.Unspecified);

    Attributes = writeAllEventMetadata switch
    {
      false => ReadOnlyCollection<AttributeKeyValue>.Empty,
      true => eventRecord.Metadata.Select(kv =>
        new AttributeKeyValue(new BxesStringValue(kv.Key), new BxesStringValue(kv.Value))).ToList()
    };
  }

  public bool Equals(IEvent? other) => other is { } && EventUtil.Equals(this, other);
}

public class BxesWriteStateWithLastEvent : BxesWriteState
{
  public EventRecordWithMetadata? LastWrittenEvent { get; set; }
}

public class OnlineBxesMethodsSerializer : OnlineMethodsSerializerBase<BxesWriteStateWithLastEvent>
{
  private const string BxesExtesnsion = ".bxes";
  
  public OnlineBxesMethodsSerializer(
    string outputDirectory, 
    Regex? targetMethodsRegex, 
    IFullMethodNameBeautifier methodNameBeautifier, 
    IProcfilerEventsFactory factory, 
    IProcfilerLogger logger,
    bool writeAllEventMetadata) 
    : base(outputDirectory, targetMethodsRegex, methodNameBeautifier, factory, logger, writeAllEventMetadata)
  {
  }

  protected override BxesWriteStateWithLastEvent? TryCreateStateInternal(EventRecordWithMetadata contextEvent)
  {
    var methodName = contextEvent.GetMethodStartEndEventInfo().Frame;
    var name = FullMethodNameBeautifier.Beautify(methodName);
    if (!name.EndsWith(BxesExtesnsion))
    {
      name += BxesExtesnsion;
    }
    
    var filePath = Path.Join(OutputDirectory, name);

    return States.GetOrCreate(
      filePath, () => new BxesWriteStateWithLastEvent { Writer = new SingleFileBxesStreamWriterImpl<BxesEvent>(filePath, 0) });
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
    SerializersUtil.DisposeWriters(States.Select(pair => (pair.Key, pair.Value.Writer)), Logger, _ => {});
  }
}