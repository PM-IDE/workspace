using System.Text;
using Core.Utils;
using Procfiler.Core.Collector;
using ProcfilerTests.Core;

namespace ProcfilerTests.Tests.SplitByMethods;

[TestFixture]
public class ByMethodSplitTests : GoldProcessBasedTest
{
  [TestCaseSource(nameof(DefaultContexts))]
  [TestCaseSource(nameof(OnlineSerializationContexts))]
  public void Test(ContextWithSolution dto) => DoTest(dto);

  private void DoTest(ContextWithSolution dto)
  {
    ExecuteTestWithGold(dto.Context, events => DumpMethodCallTree(dto.Solution.NamespaceFilterPattern, events));
  }

  private string DumpMethodCallTree(string filterPattern, CollectedEvents events)
  {
    var eventByMethods = SplitByMethodsTestUtil.SplitByMethods(events, Container, filterPattern);
    var interestingEvents = eventByMethods.OrderBy(pair => pair.Key);

    var sb = new StringBuilder();
    foreach (var (methodName, tracesOfEvents) in interestingEvents)
    {
      sb.Append("Method: ").Append(methodName).AppendNewLine().AppendNewLine();
      for (var i = 0; i < tracesOfEvents.Count; i++)
      {
        var trace = tracesOfEvents[i];
        sb.Append("Trace ").Append(i).AppendNewLine();
        sb.Append(ProgramMethodCallTreeDumper.CreateDump(trace, filterPattern));
      }

      sb.AppendNewLine();
    }

    return sb.ToString();
  }
}