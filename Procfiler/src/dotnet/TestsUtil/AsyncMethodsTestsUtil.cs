using System.Text;
using System.Text.RegularExpressions;
using Core.Events.EventRecord;
using Core.Utils;

namespace TestsUtil;

public static class AsyncMethodsTestsUtil
{
  public static string SerializeToGold(
    Dictionary<string, List<List<EventRecordWithMetadata>>> methods,
    Regex filter,
    string asyncMethodsPrefix,
    Func<EventRecordWithMetadata, string?> frameExtractor,
    Func<List<EventRecordWithMetadata>, string> traceDumper)
  {
    var sb = new StringBuilder();
    foreach (var (methodName, methodsTraces) in methods.OrderBy(pair => pair.Key))
    {
      if (!methodName.StartsWith(asyncMethodsPrefix)) continue;
      if (!filter.IsMatch(methodName)) continue;

      sb.Append(methodName);

      var allocationTraces = methodsTraces
        .Select(trace => trace.Where(e => frameExtractor(e) is { } frame && filter.IsMatch(frame)).ToList())
        .Where(t => t.Count > 0)
        .OrderBy(t => t[0].Time.QpcStamp);

      foreach (var trace in allocationTraces)
      {
        sb.AppendNewLine().Append("Trace:").AppendNewLine();
        sb.Append(traceDumper(trace));
      }

      sb.AppendNewLine().AppendNewLine();
    }

    return sb.ToString();
  }
}