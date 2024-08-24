using Core.Builder;
using Core.Collector;
using Core.CppProcfiler;
using Procfiler.Utils;

namespace Procfiler.Commands.CollectClrEvents.Context;

public readonly record struct SerializationContext(FileFormat OutputFormat);

public record struct CollectingClrEventsCommonContext(
  string OutputPath,
  SerializationContext SerializationContext,
  IParseResultInfoProvider CommandParseResult,
  string Arguments,
  ProvidersCategoryKind ProviderCategory,
  bool ClearPathBefore,
  int DurationMs,
  int MaxWaitForLogWriteTimeoutMs,
  bool PrintProcessOutput,
  string? CppProcfilerMethodsFilterRegex,
  int ProcessWaitTimeoutMs,
  CppProfilerMode CppProfilerMode,
  bool UseDuringRuntimeFiltering,
  bool CppProfilerUseConsoleLogging,
  bool ClearArtifacts,
  bool WriteAllEventMetadata,
  LogFormat LogSerializationFormat
);

public record CollectClrEventsContext(CollectingClrEventsCommonContext CommonContext);

public record CollectClrEventsFromExeContext(
  ProjectBuildInfo ProjectBuildInfo,
  CollectingClrEventsCommonContext CommonContext
) : CollectClrEventsContext(CommonContext);

public record CollectClrEventsFromExeWithRepeatContext(
  ProjectBuildInfo ProjectBuildInfo,
  int RepeatCount,
  CollectingClrEventsCommonContext CommonContext
) : CollectClrEventsFromExeContext(ProjectBuildInfo, CommonContext);

public record CollectClrEventsFromExeWithArguments(
  ProjectBuildInfo ProjectBuildInfo,
  CollectingClrEventsCommonContext CommonContext,
  IReadOnlyList<string> Arguments
) : CollectClrEventsFromExeContext(ProjectBuildInfo, CommonContext);

public record CollectClrEventsFromRunningProcessContext(
  int ProcessId,
  CollectingClrEventsCommonContext CommonContext
) : CollectClrEventsContext(CommonContext);

public record CollectClrEventsFromCommandContext(
  string CommandName,
  IReadOnlyList<string>? Arguments,
  CollectingClrEventsCommonContext CommonContext
) : CollectClrEventsContext(CommonContext);