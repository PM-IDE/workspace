using System.Diagnostics;
using System.Text;
using System.Text.RegularExpressions;
using Core.Events.EventRecord;

namespace Core.Utils;

public static class ProgramMethodCallTreeDumper
{
  public enum DumpEventKind
  {
    Start,
    End,
    Execution
  }

  public static string CreateDump(
    IEnumerable<EventRecordWithMetadata> events,
    string? pattern,
    Func<EventRecordWithMetadata, (string, DumpEventKind)?> methodInfoExtractor)
  {
    var sb = new StringBuilder();
    var regex = pattern is { } ? new Regex(pattern) : null;

    var currentIndent = 0;

    foreach (var eventRecord in events)
    {
      if (methodInfoExtractor(eventRecord) is var (frame, kind) &&
          (regex is null || regex.IsMatch(frame)))
      {
        if (kind is DumpEventKind.Start) ++currentIndent;

        if (currentIndent < 0) Debug.Fail("currentIndent < 0");

        for (var i = 0; i < currentIndent; ++i)
        {
          sb.AppendTab();
        }

        const string Start = "[start] ";
        const string End = "[ end ] ";
        const string Execution = "[exec]";

        sb.Append(kind switch
        {
          DumpEventKind.Start => Start,
          DumpEventKind.End => End,
          DumpEventKind.Execution => Execution,
          _ => throw new ArgumentOutOfRangeException()
        }).Append(frame).AppendNewLine();

        if (kind is DumpEventKind.End) --currentIndent;
      }
    }

    return sb.ToString();
  }
}