using Bxes.Models.Domain;
using Bxes.Reader;
using Bxes.Tests.Core;
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
    TestUtils.ExecuteTestWithTempFolder(log, data =>
    {
      using (var writer = new MultipleFilesBxesStreamWriterImpl<IEvent>(data.Path, log.Version, data.SystemMetadata))
      {
        foreach (var streamEvent in log.ToEventsStream())
        {
          writer.HandleEvent(streamEvent);
        }
      }

      return new MultiFileBxesReader().Read(data.Path);
    });
  }
}