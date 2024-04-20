using Bxes.Models;
using Bxes.Reader;
using Bxes.Writer;

namespace Bxes.Tests;

[TestFixture]
public class MultipleFilesSimpleWriteTest
{
  [Test]
  public void SimpleTest1()
  {
    ExecuteSimpleTest(TestLogsProvider.CreateSimpleTestLog());
  }

  private static void ExecuteSimpleTest(IEventLog log)
  {
    TestUtils.ExecuteTestWithTempFolder(log, testDirectory =>
    {
      var metadata = TestLogsProvider.GenerateRandomSystemMetadata();
      new MultipleFilesBxesWriter(metadata).Write(log, testDirectory);
      return new MultiFileBxesReader().Read(testDirectory);
    });
  }
}