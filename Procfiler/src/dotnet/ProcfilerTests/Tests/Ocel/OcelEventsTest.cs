using System.Text.Json;
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
      events => string.Join(
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
      )
    );
  }
}