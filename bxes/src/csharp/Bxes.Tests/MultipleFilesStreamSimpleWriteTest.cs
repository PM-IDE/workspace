using Bxes.Models;
using Bxes.Reader;
using Bxes.Writer.Stream;

namespace Bxes.Tests;

[TestFixture]
public class MultipleFilesStreamSimpleWriteTest
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
      using (var writer = new MultipleFilesBxesStreamWriterImpl<IEvent>(testDirectory, log.Version))
      {
        foreach (var streamEvent in log.ToEventsStream())
        {
          writer.HandleEvent(streamEvent);
        }
      }

      return new MultiFileBxesReader().Read(testDirectory);
    });
  }
}