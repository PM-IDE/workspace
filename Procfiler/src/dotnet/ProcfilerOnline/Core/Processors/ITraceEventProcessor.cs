using System.Text.RegularExpressions;
using Core.Events.EventRecord;
using Microsoft.Diagnostics.Tracing;

namespace ProcfilerOnline.Core.Processors;

public readonly ref struct CommandContext
{
  public required string ApplicationName { get; init; }
  public required Regex? TargetMethodsRegex { get; init; }
  public required ulong EventsFlushThreshold { get; init; }
  public required bool RemoveFirstMoveNextFrames { get; init; }
}

public readonly ref struct EventProcessingContext
{
  public required TraceEvent? TraceEvent { get; init; }
  public required EventRecordWithMetadata Event { get; init; }
  public required CommandContext CommandContext { get; init; }
  public required ISharedEventPipeStreamData SharedData { get; init; }
}

public interface ITraceEventProcessor
{
  void Process(EventProcessingContext context);
}