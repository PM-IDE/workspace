using Procfiler.Core.Collector;
using Procfiler.Core.Serialization.Core;
using Procfiler.Utils;

namespace Procfiler.Core.Serialization.Xes;

public class PathWriteState
{
  public required XmlWriter Writer { get; init; }
  public int TracesCount { get; set; }
}

public class NotStoringMergingTraceXesSerializer(
  IXesEventsSessionSerializer sessionSerializer,
  IProcfilerLogger logger,
  bool writeAllEventData
) : NotStoringMergingTraceSerializerBase<PathWriteState>(logger, writeAllEventData)
{
  public override void WriteTrace(string path, EventSessionInfo sessionInfo)
  {
    using var _ = new PerformanceCookie($"{GetType()}::{nameof(WriteTrace)}", Logger, LogLevel.Trace);

    if (!States.TryGetValue(path, out var existingState))
    {
      var newWriter = XmlWriter.Create(File.OpenWrite(path), new XmlWriterSettings
      {
        ConformanceLevel = ConformanceLevel.Document,
        Indent = true,
        CloseOutput = true
      });

      sessionSerializer.WriteHeader(newWriter);
      var newState = new PathWriteState { Writer = newWriter };
      States[path] = newState;
      existingState = newState;
    }

    sessionSerializer.AppendTrace(sessionInfo, existingState.Writer, existingState.TracesCount, WriteAllEventData);
    existingState.TracesCount += 1;
    existingState.Writer.Flush();
  }

  public override void Dispose()
  {
    SerializersUtil.DisposeXesWriters(States.Select(pair => (pair.Key, pair.Value.Writer)), Logger);
  }
}