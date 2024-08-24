using Core.Utils;
using TestsUtil;

namespace OnlineProcfilerTests.Core;

public abstract class OnlineProcfilerTestWithGold : OnlineProcfilerTestBase
{
  protected void Execute(Func<string> goldCreator)
  {
    var testValue = goldCreator().RemoveRn();
    GoldUtil.ExecuteGoldTest(testValue, GetType().Name, test =>
    {
      if (test.Arguments.FirstOrDefault() is KnownSolution solution)
      {
        return solution.Name;
      }

      return test.Name;
    });
  }
}