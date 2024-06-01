using Procfiler.Commands.CollectClrEvents.Split;
using ProcfilerTests.Core;
using TestsUtil;

namespace ProcfilerTests.Tests;

[TestFixture]
public class SimpleTests : ProcessTestBase
{
  [TestCaseSource(nameof(DefaultContexts))]
  [TestCaseSource(nameof(OnlineSerializationContexts))]
  public void TestSimpleManagedThreadSplitAttributes(ContextWithSolution dto)
  {
    StartProcessAndDoTestWithDefaultContext(dto.Context, events =>
    {
      Assert.That(events.Events, Has.Count.GreaterThan(KnownSolution.ConsoleApp1.ExpectedEventsCount));
      var eventsByThreads = SplitEventsHelper.SplitByKey(
        TestLogger.CreateInstance(), events.Events, SplitEventsHelper.ManagedThreadIdExtractor);

      var orderedKeys = eventsByThreads.Keys.OrderByDescending(static key => key);
      Assert.That(orderedKeys.Last(), Is.EqualTo(-1));
    });
  }

  [TestCaseSource(nameof(DefaultContexts))]
  [TestCaseSource(nameof(OnlineSerializationContexts))]
  public void TestSimpleSplitByNamesAttributes(ContextWithSolution dto)
  {
    StartProcessAndDoTestWithDefaultContext(dto.Context, events =>
    {
      Assert.That(events.Events, Has.Count.GreaterThan(KnownSolution.ConsoleApp1.ExpectedEventsCount));
      var eventsByNames =
        SplitEventsHelper.SplitByKey(TestLogger.CreateInstance(), events.Events, SplitEventsHelper.EventClassKeyExtractor);

      foreach (var (name, traceEvents) in eventsByNames)
      {
        foreach (var (_, traceEvent) in traceEvents)
        {
          Assert.That(traceEvent.EventName.Replace('/', '_'), Is.EqualTo(name));
        }
      }

      var count = eventsByNames.Select(e => e.Value.Count).Aggregate((x, y) => x + y);
      Assert.That(count, Is.EqualTo(events.Events.Count));
    });
  }
}