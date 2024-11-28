using System.Diagnostics.Tracing;

namespace ProcfilerEventSources;

[EventSource(Name = $"{nameof(MethodStartEndEventSource)}")]
public class MethodStartEndEventSource : EventSource
{
  public const int MethodStartedId = 5000;
  public const int MethodFinishedId = 5001;

  public static MethodStartEndEventSource Log { get; } = new();

  public static void LogMethodStarted(string methodName) => Log.MethodStarted(methodName);
  public static void LogMethodFinished(string methodName) => Log.MethodFinished(methodName);


  private MethodStartEndEventSource()
  {
  }


  [Event(MethodStartedId, Level = EventLevel.LogAlways)]
  public void MethodStarted(string methodName) => WriteEvent(MethodStartedId, methodName);

  [Event(MethodFinishedId, Level = EventLevel.LogAlways)]
  public void MethodFinished(string methodName) => WriteEvent(MethodFinishedId, methodName);
}