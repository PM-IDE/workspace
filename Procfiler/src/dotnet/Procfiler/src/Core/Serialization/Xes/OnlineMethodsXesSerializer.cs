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

  protected override void HandleUpdate(EventUpdateBase<PathWriterStateWithLastEvent> update)
  {
    if (update.FrameInfo.State is null) return;

    switch (update)
    {
      case MethodExecutionUpdate<PathWriterStateWithLastEvent> methodExecutionUpdate:
        HandleMethodExecutionEvent(methodExecutionUpdate);
        break;
      case MethodFinishedUpdate<PathWriterStateWithLastEvent> methodFinishedUpdate:
        HandleMethodFinishedEvent(methodFinishedUpdate);
        break;
      case MethodStartedUpdate<PathWriterStateWithLastEvent> methodStartedUpdate:
        HandleMethodStartEvent(methodStartedUpdate);
        break;
      case NormalEventUpdate<PathWriterStateWithLastEvent> normalEventUpdate:
        HandleNormalEvent(normalEventUpdate);
        break;
      default:
        throw new ArgumentOutOfRangeException(nameof(update));
    }
  }

  private void HandleMethodStartEvent(MethodStartedUpdate<PathWriterStateWithLastEvent> methodStartedUpdate)
  {
    var state = methodStartedUpdate.FrameInfo.State!;

    MethodNames.Add(methodStartedUpdate.FrameInfo.Frame);
    sessionSerializer.WriteTraceStart(state.Writer, state.TracesCount);
    state.TracesCount++;
  }

  private void WriteEvent(PathWriterStateWithLastEvent state, EventRecordWithMetadata eventRecord)
  {
    state.LastWrittenEvent = eventRecord;
    sessionSerializer.WriteEvent(eventRecord, state.Writer, WriteAllEventMetadata);
  }

  private void HandleMethodFinishedEvent(MethodFinishedUpdate<PathWriterStateWithLastEvent> methodFinishedUpdate)
  {
    var state = methodFinishedUpdate.FrameInfo.State!;
    state.Writer.WriteEndElement();
  }

  private void HandleMethodExecutionEvent(MethodExecutionUpdate<PathWriterStateWithLastEvent> methodExecutionUpdate)
  {
    var state = methodExecutionUpdate.FrameInfo.State;
    var executionEvent = CurrentFrameInfoUtil.CreateMethodExecutionEvent(
      methodExecutionUpdate.FrameInfo,
      Factory,
      methodExecutionUpdate.MethodName,
      state!.LastWrittenEvent);

    WriteEvent(state, executionEvent);
  }

  private void HandleNormalEvent(NormalEventUpdate<PathWriterStateWithLastEvent> normalEventUpdate)
  {
    WriteEvent(normalEventUpdate.FrameInfo.State!, normalEventUpdate.Event);
  }

  public override void Dispose()
  {
    SerializersUtil.DisposeXesWriters(States.Select(pair => (pair.Key, pair.Value.Writer)), Logger);
  }
}