using System.Text.Json;
using ProcfilerTests.Core;
using TestsUtil;

namespace ProcfilerTests.Tests.ProcfilerEventPipeLoggerTests;

public class ProcfilerEventPipeLoggerTest : GoldProcessBasedTest
{
  [Test]
  public void DoTest()
  {
    ExecuteTestWithGold(KnownSolution.ProcfilerEventPipeLogger.CreateDefaultContext(), events =>
    {
      return string.Join(
        "\n", 
        events.Events
          .Where(e => e.Event.EventName is "BusinessEvent")
          .Select(e => e.Event)
          .OrderBy(e => e.Time.QpcStamp)
          .Select(e => $"{e.EventName} {JsonSerializer.Serialize(e.Metadata)}")
      );
    });
  }
}