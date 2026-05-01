using Core.Utils;
using TestsUtil;

namespace OnlineProcfilerTests.Core;

public abstract class OnlineProcfilerTestWithGold : OnlineProcfilerTestBase
{
  protected void Execute(Func<string> goldCreator)
  {
    var testValue = goldCreator().RemoveRn();
    GoldUtil.ExecuteGoldTest(testValue, GetType().Name, () =>
    {
      return TestContext.CurrentContext.Test.Arguments.FirstOrDefault() switch
      {
        KnownSolution solution => solution.Name,
        _ => TestContext.CurrentContext.Test.Name
      };
    });
  }
}