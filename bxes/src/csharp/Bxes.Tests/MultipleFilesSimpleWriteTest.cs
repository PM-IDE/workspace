using Bxes.Models.Domain;
using Bxes.Reader;
using Bxes.Tests.Core;
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
    TestUtils.ExecuteTestWithTempFolder(log, data =>
    {
      new MultipleFilesBxesWriter(data.SystemMetadata).Write(log, data.Path);
      return new MultiFileBxesReader().Read(data.Path);
    });
  }
}