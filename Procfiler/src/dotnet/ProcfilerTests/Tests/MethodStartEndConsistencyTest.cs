using Autofac;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.EventsProcessing;
using Procfiler.Utils;
using ProcfilerTests.Core;

namespace ProcfilerTests.Tests;

[TestFixture]
public class MethodStartEndConsistencyTest : ProcessTestBase
{
  [TestCaseSource(nameof(DefaultContexts))]
  [TestCaseSource(nameof(OnlineSerializationContexts))]
  public void Test(ContextWithSolution dto) => DoTest(dto);

  private void DoTest(ContextWithSolution dto)
  {
    StartProcessAndDoTestWithDefaultContext(dto.Context, events =>
    {
      var globalData = events.GlobalData;
      var eventsByThreads = SplitEventsHelper.SplitByKey(
        TestLogger.CreateInstance(), events.Events, SplitEventsHelper.ManagedThreadIdExtractor);
      eventsByThreads.Remove(-1);

      foreach (var (threadId, eventsForThread) in eventsByThreads)
      {
        var processor = Container.Resolve<IUnitedEventsProcessor>();
        processor.ApplyMultipleMutators(eventsForThread, globalData, EmptyCollections<Type>.EmptySet);
        TestUtil.CheckMethodConsistencyOrThrow(threadId, eventsForThread, globalData, Container);
      }
    });
  }
}