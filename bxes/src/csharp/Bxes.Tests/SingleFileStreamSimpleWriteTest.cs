using Bxes.Models.Domain;
using Bxes.Reader;
using Bxes.Writer.Stream;

namespace Bxes.Tests;

[TestFixture]
public class SingleFileStreamSimpleWriteTest
{
  [Test]
  public void SimpleTest1()
  {
    ExecuteSimpleTest(TestLogsProvider.CreateSimpleTestLog());
  }

  private static void ExecuteSimpleTest(IEventLog log)
  {
    TestUtils.ExecuteTestWithTempFile(log, testFile =>
    {
      using (var writer = new SingleFileBxesStreamWriterImpl<IEvent>(testFile, log.Version))
      {
        foreach (var streamEvent in log.ToEventsStream())
        {
          writer.HandleEvent(streamEvent);
        }
      }

      return new SingleFileBxesReader().Read(testFile);
    });
  }
}