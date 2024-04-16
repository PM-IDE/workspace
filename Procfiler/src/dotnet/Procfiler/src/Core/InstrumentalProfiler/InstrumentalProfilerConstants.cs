using ProcfilerEventSources;

namespace Procfiler.Core.InstrumentalProfiler;

public static class InstrumentalProfilerConstants
{
  public const string ProcfilerEventSource = "ProcfilerEventSources";
  public const string MethodStartEndEventSourceType = $"{ProcfilerEventSource}.{nameof(MethodStartEndEventSource)}";

  public const string LogMethodStartedMethodName = nameof(MethodStartEndEventSource.LogMethodStarted);
  public const string LogMethodFinishedMethodName = nameof(MethodStartEndEventSource.LogMethodFinished);
}