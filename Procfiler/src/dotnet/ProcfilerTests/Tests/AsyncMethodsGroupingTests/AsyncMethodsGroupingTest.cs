using System.Text;
using System.Text.RegularExpressions;
using Autofac;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsProcessing;
using Procfiler.Core.SplitByMethod;
using ProcfilerTests.Core;
using TestsUtil;

namespace ProcfilerTests.Tests.AsyncMethodsGroupingTests;

[TestFixture]
public class AsyncMethodsGroupingTest : GoldProcessBasedTest
{
  [Test] public void TestNotSimpleAsync() => DoSimpleTest(KnownSolution.NotSimpleAsyncAwait);
  [Test] public void TestSimpleAsyncAwait() => DoSimpleTest(KnownSolution.SimpleAsyncAwait);
  [Test] public void TestAsyncAwait() => DoSimpleTest(KnownSolution.AsyncAwait);
  [Test] public void TestAsyncAwaitTaskFactoryNew() => DoSimpleTest(KnownSolution.AsyncAwaitTaskFactoryNew);
  [Test] public void TestAwaitForeach() => DoSimpleTest(KnownSolution.AwaitForeach);
  [Test] public void TestAsyncDisposable() => DoSimpleTest(KnownSolution.AsyncDisposable);


  private void DoSimpleTest(KnownSolution solution)
  {
    ExecuteTestWithGold(
      solution.CreateDefaultContext(),
      events => ExecuteAsyncGroupingTest(events, solution));
  }

  private string ExecuteAsyncGroupingTest(CollectedEvents events, KnownSolution knownSolution)
  {
    var processingContext = EventsProcessingContext.DoEverything(events.Events, events.GlobalData);
    Container.Resolve<IUnitedEventsProcessor>().ProcessFullEventLog(processingContext);

    var splitter = Container.Resolve<IByMethodsSplitter>();

    var splitContext = new SplitContext(events, string.Empty, InlineMode.EventsAndMethodsEvents, false, true);
    var methods = splitter.Split(splitContext);
    var asyncMethodsPrefix = Container.Resolve<IAsyncMethodsGrouper>().AsyncMethodsPrefix;
    var filter = new Regex(knownSolution.NamespaceFilterPattern);

    return AsyncMethodsTestsUtil.SerializeToGold(methods, filter, asyncMethodsPrefix, e => e.TryGetMethodStartEndEventInfo()?.Frame,
      trace => TestsMethodCallTreeDumper.CreateDump(trace, null));
  }
}