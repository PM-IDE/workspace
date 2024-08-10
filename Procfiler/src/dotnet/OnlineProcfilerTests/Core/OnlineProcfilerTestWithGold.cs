using TestsUtil;

namespace OnlineProcfilerTests.Core;

public abstract class OnlineProcfilerTestWithGold : OnlineProcfilerTestBase
{
  protected void Execute(Func<string> goldCreator)
  {
    var testValue = goldCreator();
    GoldUtil.ExecuteGoldTest(testValue, GetType().Name, test => test.Name);
  }
}