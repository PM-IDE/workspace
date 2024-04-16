using System.Text;
using System.Text.RegularExpressions;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsCollection;
using Procfiler.Utils;

namespace ProcfilerTests.Core;

public static class ProgramMethodCallTreeDumper
{
  public static string CreateDump(IEnumerable<EventRecordWithPointer> events, string pattern)
  {
    return CreateDump(events.Select(pair => pair.Event), pattern);
  }

  public static string CreateDump(IEnumerable<EventRecordWithMetadata> events, string pattern)
  {
    var sb = new StringBuilder();
    var regex = new Regex(pattern);
    var currentIndent = 0;

    foreach (var eventRecord in events)
    {
      if (eventRecord.TryGetMethodStartEndEventInfo() is var (frame, isStart) &&
          regex.IsMatch(frame))
      {
        if (isStart) ++currentIndent;

        if (currentIndent < 0) Assert.Fail();

        for (var i = 0; i < currentIndent; ++i)
        {
          sb.AppendTab();
        }

        const string Start = "[start] ";
        const string End = "[ end ] ";
        sb.Append(isStart ? Start : End).Append(frame).AppendNewLine();

        if (!isStart) --currentIndent;
      }
    }

    return sb.ToString();
  }
}