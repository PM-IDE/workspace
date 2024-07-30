using System.Text.RegularExpressions;
using Core.Events.EventRecord;

namespace ProcfilerOnline.Core.Processors;

public readonly ref struct CommandContext
{
  public required Regex? TargetMethodsRegex { get; init; }
}

public readonly ref struct EventProcessingContext
{
  public required EventRecordWithMetadata Event { get; init; }
  public required CommandContext CommandContext { get; init; }
}

public interface ITraceEventProcessor
{
  void Process(EventProcessingContext context);
}

public interface ISharedDataUpdater : ITraceEventProcessor;