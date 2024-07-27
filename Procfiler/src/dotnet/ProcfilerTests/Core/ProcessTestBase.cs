using Autofac;
using Procfiler.Commands.CollectClrEvents;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord.EventsCollection;

namespace ProcfilerTests.Core;

public abstract class ProcessTestBase : TestWithContainerBase
{
  protected void StartProcessAndDoTestWithDefaultContext(CollectClrEventsFromExeContext context, Action<CollectedEvents> testAction)
  {
    StartProcessAndDoTest(context, testAction);
  }

  protected void StartProcessAndDoTest(CollectClrEventsContext context, Action<CollectedEvents> testAction)
  {
    Container.Resolve<ICommandExecutorDependantOnContext>().Execute(context, testAction);
  }

  protected void StartProcessSplitEventsByThreadsAndDoTest(
    CollectClrEventsFromExeContext context, Action<Dictionary<long, IEventsCollection>, SessionGlobalData> testFunc)
  {
    StartProcessAndDoTestWithDefaultContext(context, events =>
    {
      var extractor = SplitEventsHelper.ManagedThreadIdExtractor;
      var eventsByThreads = SplitEventsHelper.SplitByKey(TestLogger.CreateInstance(), events.Events, extractor);

      testFunc(eventsByThreads, events.GlobalData);
    });
  }
}