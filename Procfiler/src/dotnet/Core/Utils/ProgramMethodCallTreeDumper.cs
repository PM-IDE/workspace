using System.Diagnostics;
using System.Text;
using System.Text.RegularExpressions;
using Core.Events.EventRecord;
using Core.Utils;

namespace ProcfilerTests.Core;

public static class ProgramMethodCallTreeDumper
{
  public static string CreateDump(
    IEnumerable<EventRecordWithMetadata> events,
    string? pattern,
    Func<EventRecordWithMetadata, (string, bool)?> methodInfoExtractor)
  {
    var sb = new StringBuilder();
    var regex = pattern is { } ? new Regex(pattern) : null;

    var currentIndent = 0;

    foreach (var eventRecord in events)
    {
      if (methodInfoExtractor(eventRecord) is var (frame, isStart) &&
          (regex is null || regex.IsMatch(frame)))
      {
        if (isStart) ++currentIndent;

        if (currentIndent < 0) Debug.Fail("currentIndent < 0");

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