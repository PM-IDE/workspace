using Microsoft.Diagnostics.Tracing;

namespace ProcfilerOnline.Core;

public enum MethodKind
{
  Begin,
  End
}

public static class TraceEventExtensions
{
  private const string CppMethodStartEventName = "ProcfilerMethod/Begin";
  private const string CppMethodFinishedEventName = "ProcfilerMethod/End";


  public static MethodKind GetMethodEventKind(this TraceEvent traceEvent) => traceEvent.EventName switch
  {
    CppMethodStartEventName => MethodKind.Begin,
    CppMethodFinishedEventName => MethodKind.End,
    _ => throw new ArgumentOutOfRangeException()
  };
}