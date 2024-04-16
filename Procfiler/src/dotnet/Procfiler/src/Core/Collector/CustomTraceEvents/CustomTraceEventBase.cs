using Procfiler.Core.Constants.TraceEvents;

namespace Procfiler.Core.Collector.CustomTraceEvents;

public abstract class CustomTraceEventBase : TraceEvent
{
  protected override Delegate Target { get; set; } = () => { };

  public override string[] PayloadNames => payloadNames;


  protected CustomTraceEventBase(TraceEvent traceEvent, string newName, string[] payloadNames)
    : base(
      (ushort)traceEvent.ID,
      (int)traceEvent.Task,
      traceEvent.TaskName,
      traceEvent.TaskGuid,
      (int)traceEvent.Opcode,
      traceEvent.OpcodeName,
      traceEvent.ProviderGuid,
      traceEvent.ProcessName
    )
  {
    this.payloadNames = payloadNames;

    Debug.Assert((int)ID == UnknownEventsIds.GcLohCompactId);

    var userDataField = GetFieldInfo("userData");
    userDataField.SetValue(this, traceEvent.DataStart);

    GetFieldValueAndSetToThis("eventRecord", traceEvent);
    GetFieldValueAndSetToThis("traceEventSource", traceEvent);
    GetFieldValueAndSetToThis("eventIndex", traceEvent);

    var eventNameField = GetFieldInfo("eventName");
    eventNameField.SetValue(this, newName);
  }

  private void GetFieldValueAndSetToThis(string fieldName, TraceEvent sourceEvent)
  {
    var field = GetFieldInfo(fieldName);
    field.SetValue(this, field.GetValue(sourceEvent));
  }

  private FieldInfo GetFieldInfo(string fieldName)
  {
    var fieldInfo = GetType().GetField(fieldName, BindingFlags.Instance | BindingFlags.NonPublic);
    Debug.Assert(fieldInfo is { });
    return fieldInfo;
  }
}