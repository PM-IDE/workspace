using Bxes.Models.Domain;
using Bxes.Reader;
using Bxes.Writer;

namespace Bxes.Tests;

[TestFixture]
public class SingleFileSimpleWriteTest
{
  [Test]
  public void SimpleTest1()
  {
    ExecuteSimpleTest(TestLogsProvider.CreateSimpleTestLog());
  }

  private static void ExecuteSimpleTest(IEventLog log)
  {
    TestUtils.ExecuteTestWithTempFile(log, testPath =>
    {
      var metadata = TestLogsProvider.GenerateRandomSystemMetadata();
      new SingleFileBxesWriter(metadata).Write(log, testPath);
      return new SingleFileBxesReader().Read(testPath);
    });
  }
}