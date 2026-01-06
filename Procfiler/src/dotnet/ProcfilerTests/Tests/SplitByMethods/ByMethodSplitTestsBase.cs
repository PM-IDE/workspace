using System.Text;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.Collector;
using ProcfilerTests.Core;

namespace ProcfilerTests.Tests.SplitByMethods;

public abstract class ByMethodSplitTestsBase : GoldProcessBasedTest
{
  protected abstract InlineMode InlineMode { get; }


  [TestCaseSource(nameof(DefaultContexts))]
  [TestCaseSource(nameof(OnlineSerializationContexts))]
  public void DoTest(ContextWithSolution dto)
  {
    ExecuteTestWithGold(dto.Context,
      events => DumpMethodCallTree(dto.Solution.NamespaceFilterPattern, events, InlineMode));
  }

  private string DumpMethodCallTree(string filterPattern, CollectedEvents events, InlineMode inlineMode)
  {
    var eventByMethods = SplitByMethodsTestUtil.SplitByMethods(events, Container, filterPattern, inlineMode);
    var interestingEvents = eventByMethods.OrderBy(pair => pair.Key);

    var sb = new StringBuilder();
    foreach (var (methodName, tracesOfEvents) in interestingEvents)
    {
      sb.Append("Method: ").Append(methodName).AppendNewLine().AppendNewLine();
      for (var i = 0; i < tracesOfEvents.Count; i++)
      {
        var trace = tracesOfEvents[i];
        sb.Append("Trace ").Append(i).AppendNewLine();
        sb.Append(TestsMethodCallTreeDumper.CreateDump(trace, filterPattern));
      }

      sb.AppendNewLine();
    }

    return sb.ToString();
  }
}

[TestFixture]
public class ByMethodSplitTests : ByMethodSplitTestsBase
{
  protected override InlineMode InlineMode => InlineMode.EventsAndMethodsEvents;
}

[TestFixture]
public class ByMethodSplitTestsNoInline : ByMethodSplitTestsBase
{
  protected override InlineMode InlineMode => InlineMode.NotInline;
}