using Bxes.IntegrationTests.BxesImplExecutors;

namespace Bxes.IntegrationTests;

[TestFixture]
public class DifferentImplXesToBxesTest
{
  private readonly List<IBxesImplExecutor> myExecutors = new()
  {
    new RustFicusImplExecutor(),
    new CSharpImplExecutor()
  };


  [Test]
  public void ExecuteTest()
  {
    foreach (var directory in Directory.GetDirectories(TestDataProvider.SourceLogDirectory))
    {
      foreach (var xesFile in Directory.EnumerateFiles(directory))
      {
        GoldBasedTestExecutor.Execute(myExecutors, xesFile);
      }
    }
  }
}