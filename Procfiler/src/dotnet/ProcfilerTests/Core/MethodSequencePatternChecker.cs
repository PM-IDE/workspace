using System.Collections;
using System.Text.RegularExpressions;
using Procfiler.Core.EventRecord.EventRecord;

namespace ProcfilerTests.Core;

public record MethodInvocationTree(string NamePattern) : IEnumerable
{
  public List<MethodInvocationTree> InnerCalls { get; } = [];


  public void Add(MethodInvocationTree tree) => InnerCalls.Add(tree);

  public IEnumerator GetEnumerator() => InnerCalls.GetEnumerator();
}

public static class MethodSequencePatternChecker
{
  public static bool ContainsSequence(
    IEnumerable<EventRecordWithMetadata> events,
    MethodInvocationTree invocationTree,
    string globalMethodFilterPattern)
  {
    var filterRegex = new Regex(globalMethodFilterPattern);
    var methodsStartOrEnd = events
      .Where(e => e.IsMethodStartOrEndEvent())
      .Select(e => e.GetMethodStartEndEventInfo())
      .Where(info => filterRegex.IsMatch(info.Frame))
      .ToList();

    var firstIndex = methodsStartOrEnd.FindIndex(e => AreFrameNamesEqual(e.Frame, invocationTree.NamePattern));
    if (firstIndex != 0) return false;

    var lastIndex = methodsStartOrEnd.FindIndex(e => AreFrameNamesEqual(e.Frame, invocationTree.NamePattern));
    if (lastIndex != firstIndex) return false;

    return DoCheckContainsSequence(methodsStartOrEnd, ref firstIndex, invocationTree);
  }

  private static bool AreFrameNamesEqual(string frame, string regex) => new Regex(regex).IsMatch(frame);

  private static bool DoCheckContainsSequence(
    List<EventRecordExtensions.MethodStartEndEventInfo> list, ref int index, MethodInvocationTree invocation)
  {
    var (frame, isStart) = list[index];
    if (!isStart || !AreFrameNamesEqual(frame, invocation.NamePattern)) return false;

    foreach (var innerInvocation in invocation.InnerCalls)
    {
      ++index;
      DoCheckContainsSequence(list, ref index, innerInvocation);
    }

    ++index;
    var endOfMethod = list[index];
    return !endOfMethod.IsStart && AreFrameNamesEqual(endOfMethod.Frame, invocation.NamePattern);
  }
}