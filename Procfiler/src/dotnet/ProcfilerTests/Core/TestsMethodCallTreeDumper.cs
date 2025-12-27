using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Core.EventRecord;

namespace ProcfilerTests.Core;

public static class TestsMethodCallTreeDumper
{
  public static string CreateDump(IEnumerable<EventRecordWithMetadata> trace, string? filterPattern)
  {
    return ProgramMethodCallTreeDumper.CreateDump(trace, filterPattern, e =>
    {
      if (e.TryGetMethodStartEndEventInfo() is var (frame, isStart))
      {
        return (frame, isStart switch
        {
          true => ProgramMethodCallTreeDumper.DumpEventKind.Start,
          false => ProgramMethodCallTreeDumper.DumpEventKind.End
        });
      }

      if (e.IsMethodExecutionEvent(out frame))
      {
        return (frame!, ProgramMethodCallTreeDumper.DumpEventKind.Execution);
      }

      return null;
    });
  }
}