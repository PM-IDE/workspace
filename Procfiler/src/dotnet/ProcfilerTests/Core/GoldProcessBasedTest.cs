using System.Text;
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
      var testValue = testFunc(events).RemoveRn();

      GoldUtil.ExecuteGoldTest(testValue, folderName, ExtractTestName);
    });
  }

  private static string ExtractTestName(TestContext.TestAdapter test)
  {
    if (test.Arguments.FirstOrDefault() is ContextWithSolution dto)
    {
      return dto.Solution.Name;
    }

    return test.Name;
  }
}