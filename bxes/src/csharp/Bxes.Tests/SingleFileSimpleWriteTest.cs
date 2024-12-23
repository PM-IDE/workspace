using Bxes.Models.Domain;
using Bxes.Reader;
using Bxes.Tests.Core;
using Bxes.Utils;
using Bxes.Writer;

namespace Bxes.Tests;

[TestFixture]
public class SingleFileSimpleWriteTest
{
  [Test]
  public void SimpleTest1()
  {
    ExecuteSimpleTest(RandomLogsGenerator.CreateSimpleLog(Defaults.DefaultRandomLogGenerationParameters));
  }

  private static void ExecuteSimpleTest(IEventLog log)
  {
    TestUtils.ExecuteTestWithTempFile(log, data =>
    {
      new SingleFileBxesWriter(data.SystemMetadata).Write(log, data.Path);
      return new SingleFileBxesReader().Read(data.Path);
    });
  }
}