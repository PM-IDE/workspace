using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Core.EventRecord;
using Procfiler.Core.Serialization.Core;
using Procfiler.Core.SplitByMethod;

namespace Procfiler.Core.Serialization.Xes;

public class PathWriterStateWithLastEvent : PathWriteState
{
  public EventRecordWithMetadata? LastWrittenEvent { get; set; }
}

public class OnlineMethodsXesSerializer(
  string outputDirectory,
  Regex? targetMethodsRegex,
  IXesEventsSessionSerializer sessionSerializer,
  IFullMethodNameBeautifier methodNameBeautifier,
  IProcfilerEventsFactory factory,
  IProcfilerLogger logger,
  bool writeAllEventMetadata)
  : OnlineMethodsSerializerBase<PathWriterStateWithLastEvent>(
    outputDirectory, targetMethodsRegex, methodNameBeautifier, factory, logger, writeAllEventMetadata)
{
  protected override PathWriterStateWithLastEvent? TryCreateStateInternal(EventRecordWithMetadata contextEvent)
  {
    var methodName = contextEvent.GetMethodStartEndEventInfo().Frame;
    var name = FullMethodNameBeautifier.Beautify(methodName);
    if (!name.EndsWith(SerializersUtil.XesExtension))
    {
      name += SerializersUtil.XesExtension;
    }

    var filePath = Path.Join(OutputDirectory, name);

    return States.GetOrCreate(filePath, () =>
    {
      var outputStream = File.OpenWrite(filePath);
      var writer = XmlWriter.Create(outputStream, new XmlWriterSettings
      {
        ConformanceLevel = ConformanceLevel.Document,
        Indent = true,
        CloseOutput = true
      });

      sessionSerializer.WriteHeader(writer);
      return new PathWriterStateWithLastEvent { Writer = writer };
    });
  }

  public override void HandleUpdate(EventUpdateBase update)
  {
    if (update.FrameInfo.State is not PathWriterStateWithLastEvent state) return;

    switch (update)
    {
      case MethodExecutionUpdate methodExecutionUpdate:
        HandleMethodExecutionEvent(methodExecutionUpdate);
        break;
      case MethodFinishedUpdate:
        HandleMethodFinishedEvent(state);
        break;
      case MethodStartedUpdate:
        HandleMethodStartEvent(state);
        break;
      case NormalEventUpdate normalEventUpdate:
        HandleNormalEvent(state, normalEventUpdate.Event);
        break;
      default:
        throw new ArgumentOutOfRangeException(nameof(update));
    }
  }

  private void HandleMethodStartEvent(PathWriterStateWithLastEvent state)
  {
    sessionSerializer.WriteTraceStart(state.Writer, state.TracesCount);
    state.TracesCount++;
  }

  private void WriteEvent(PathWriterStateWithLastEvent state, EventRecordWithMetadata eventRecord)
  {
    state.LastWrittenEvent = eventRecord;
    sessionSerializer.WriteEvent(eventRecord, state.Writer, WriteAllEventMetadata);
  }

  private static void HandleMethodFinishedEvent(PathWriterStateWithLastEvent state)
  {
    state.Writer.WriteEndElement();
  }

  private void HandleMethodExecutionEvent(MethodExecutionUpdate methodExecutionUpdate)
  {
    var state = (PathWriterStateWithLastEvent)methodExecutionUpdate.FrameInfo.State!;

    var executionEvent = CurrentFrameInfoUtil.CreateMethodExecutionEvent(
      methodExecutionUpdate.FrameInfo.Frame,
      Factory,
      methodExecutionUpdate.MethodName,
      state.LastWrittenEvent
    );

    WriteEvent(state, executionEvent);
  }

  private void HandleNormalEvent(PathWriterStateWithLastEvent state, EventRecordWithMetadata @event)
  {
    WriteEvent(state, @event);
  }

  public override void Dispose()
  {
    SerializersUtil.DisposeXesWriters(States.Select(pair => (pair.Key, pair.Value.Writer)), Logger);
  }
}