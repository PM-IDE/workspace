using System.Text.RegularExpressions;
using Autofac;
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
  private static readonly IEnumerable<KnownSolution> ourAsyncSolutions = KnownSolution.AsyncSolutions;


  [TestCaseSource(nameof(ourAsyncSolutions))]
  public void DoTest(KnownSolution solution)
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

    var splitContext = new SplitContext(events, string.Empty, InlineMode.EventsAndMethodsEvents, false, true, false);
    var methods = splitter.Split(splitContext);
    var asyncMethodsPrefix = Container.Resolve<IAsyncMethodsGrouper>().AsyncMethodsPrefix;
    var filter = new Regex(knownSolution.NamespaceFilterPattern);

    return MethodsTestsUtil.SerializeToGold(methods, filter, asyncMethodsPrefix, e => e.TryGetMethodStartEndEventInfo()?.Frame,
      trace => TestsMethodCallTreeDumper.CreateDump(trace, null));
  }
}