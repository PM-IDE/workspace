using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Core.Collector;
using TestsUtil;

namespace ProcfilerTests.Core;

public class GoldProcessBasedTest : ProcessTestBase
{
  protected void ExecuteTestWithGold(CollectClrEventsFromExeContext context, Func<CollectedEvents, string> testFunc)
  {
    StartProcessAndDoTestWithDefaultContext(context, events =>
    {
      var folderName = GetType().Name;
      if (TryGetFramework() is { } tfm)
      {
        folderName = Path.Combine(folderName, tfm);
      }

      var testValue = testFunc(events).RemoveRn();

      GoldUtil.ExecuteGoldTest(testValue, folderName, ExtractTestName);
    });
  }

  private static object? TryGetFirstArg() => TestContext.CurrentContext.Test.Arguments.FirstOrDefault();

  private static string? TryGetFramework() => TryGetFirstArg() switch
  {
    ContextWithSolution dto => dto.Solution.Tfm,
    _ => null
  };

  private static string ExtractTestName() => TryGetFirstArg() switch
  {
    ContextWithSolution dto => dto.Solution.Name,
    _ => TestContext.CurrentContext.Test.Name
  };
}