using Core.Events.EventRecord;
using Procfiler.Core.EventRecord;

namespace ProcfilerTests.Core;

public static class TestsMethodCallTreeDumper
{
  public static string CreateDump(IEnumerable<EventRecordWithMetadata> trace, string? filterPattern)
  {
    return ProgramMethodCallTreeDumper.CreateDump(trace, filterPattern, e => e.TryGetMethodStartEndEventInfo() switch
    {
      var (frame, isStart) => (frame, isStart),
      _ => null
    });
  }
}