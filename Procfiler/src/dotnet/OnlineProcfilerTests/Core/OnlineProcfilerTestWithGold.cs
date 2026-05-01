using Core.Utils;
using TestsUtil;

namespace OnlineProcfilerTests.Core;

public abstract class OnlineProcfilerTestWithGold : OnlineProcfilerTestBase
{
  protected void Execute(Func<string> goldCreator)
  {
    var testValue = goldCreator().RemoveRn();
    var folderName = GetType().Name;
    var solution = TestContext.CurrentContext.Test.Arguments.FirstOrDefault() as KnownSolution;

    if (solution is { })
    {
      folderName = Path.Combine(folderName, solution.Tfm);
    }

    GoldUtil.ExecuteGoldTest(testValue, folderName, () =>
    {
      return solution switch
      {
        { } => solution.Name,
        _ => TestContext.CurrentContext.Test.Name
      };
    });
  }
}