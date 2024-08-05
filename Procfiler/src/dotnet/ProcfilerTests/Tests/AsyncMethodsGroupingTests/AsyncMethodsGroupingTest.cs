using System.Text;
using System.Text.RegularExpressions;
using Autofac;
using Core.Events.EventRecord;
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
  [Test]
  public void TestNotSimpleAsync() => DoSimpleTest(KnownSolution.NotSimpleAsyncAwait);

  [Test]
  public void TestSimpleAsyncAwait() => DoSimpleTest(KnownSolution.SimpleAsyncAwait);


  private void DoSimpleTest(KnownSolution solution)
  {
    ExecuteTestWithGold(
      solution.CreateDefaultContext(),
      events => ExecuteAsyncGroupingTest(events, solution, DumpFrames));
  }

  private static string DumpFrames(IReadOnlyList<EventRecordWithMetadata> events)
  {
    var sb = new StringBuilder();
    foreach (var eventRecord in events)
    {
      sb.Append(eventRecord.EventName).AppendNewLine();
    }

    return sb.ToString();
  }

  private string ExecuteAsyncGroupingTest(
    CollectedEvents events,
    KnownSolution knownSolution,
    Func<IReadOnlyList<EventRecordWithMetadata>, string> tracesDumber)
  {
    var processingContext = EventsProcessingContext.DoEverything(events.Events, events.GlobalData);
    Container.Resolve<IUnitedEventsProcessor>().ProcessFullEventLog(processingContext);

    var splitter = Container.Resolve<IByMethodsSplitter>();

    var splitContext = new SplitContext(events, string.Empty, InlineMode.EventsAndMethodsEvents, false, true);
    var methods = splitter.Split(splitContext);
    var asyncMethodsPrefix = Container.Resolve<IAsyncMethodsGrouper>().AsyncMethodsPrefix;

    var sb = new StringBuilder();
    var filter = new Regex(knownSolution.NamespaceFilterPattern);

    foreach (var (methodName, methodsTraces) in methods)
    {
      if (!methodName.StartsWith(asyncMethodsPrefix)) continue;
      if (!filter.IsMatch(methodName)) continue;

      sb.Append(methodName);

      var allocationTraces = methodsTraces
        .Select(trace => trace.Where(e => e.TryGetMethodStartEndEventInfo() is { Frame: var frame } && filter.IsMatch(frame)).ToList())
        .Where(t => t.Count > 0)
        .OrderBy(t => t[0].Time.QpcStamp);

      foreach (var trace in allocationTraces)
      {
        sb.AppendNewLine().Append("Trace:").AppendNewLine();
        sb.Append(ProgramMethodCallTreeDumper.CreateDump(trace, null));
      }

      sb.AppendNewLine().AppendNewLine();
    }

    return sb.ToString();
  }
}