using System.Text.Json;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord.EventsCollection;
using ProcfilerTests.Core;
using TestsUtil;

namespace ProcfilerTests.Tests.Ocel;

public class OcelEventsTest : GoldProcessBasedTest
{
  [Test]
  public void SimpleTest()
  {
    ExecuteTestWithGold(
      KnownSolution.Ocel.CreateOnlineSerializationContext(),
      SerializeOcelEvents
    );
  }

  [Test]
  public void SimpleTest2()
  {
    ExecuteTestWithGold(
      KnownSolution.Ocel2.CreateOnlineSerializationContext(),
      SerializeOcelEvents
    );
  }

  private static string SerializeOcelEvents(CollectedEvents events) => string.Join(
    "\n",
    events.Events
      .Where(e => e.Event.EventClass.StartsWith("Ocel"))
      .Select(e => e.Event)
      .OrderBy(e => e.Time.QpcStamp)
      .Select(e =>
      {
        //its unique Guid so remove it for consistency
        e.Metadata.Remove("activityId");
        return $"{e.EventName} {JsonSerializer.Serialize(e.Metadata)}";
      })
  );
}