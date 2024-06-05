using System.Text;
using System.Text.RegularExpressions;
using Autofac;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsProcessing;
using Procfiler.Core.SplitByMethod;
using Procfiler.Utils;
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
      events => ExecuteAsyncGroupingTest(events, solution, ExtractAllocations, DumpsAllocationsWith));
  }

  private static List<EventRecordWithMetadata> ExtractAllocations(IReadOnlyList<EventRecordWithMetadata> events)
  {
    var regex = new Regex("[a-zA-Z]+.Class[0-9]");
    List<EventRecordWithMetadata> allocations = [];
    foreach (var eventRecord in events)
    {
      if (!eventRecord.IsGcSampledObjectAlloc(out var typeName)) continue;
      if (regex.Match(typeName).Length != typeName.Length) continue;

      allocations.Add(eventRecord);
    }

    return allocations;
  }

  private static string DumpsAllocationsWith(IReadOnlyList<EventRecordWithMetadata> events)
  {
    var sb = new StringBuilder();
    foreach (var eventRecord in events)
    {
      sb.Append(eventRecord.GetAllocatedTypeNameOrThrow()).AppendNewLine();
    }

    return sb.ToString();
  }

  private string ExecuteAsyncGroupingTest(
    CollectedEvents events,
    KnownSolution knownSolution,
    Func<IReadOnlyList<EventRecordWithMetadata>, List<EventRecordWithMetadata>> allocationsExtractor,
    Func<IReadOnlyList<EventRecordWithMetadata>, string> tracesDumber)
  {
    var processingContext = EventsProcessingContext.DoEverything(events.Events, events.GlobalData);
    Container.Resolve<IUnitedEventsProcessor>().ProcessFullEventLog(processingContext);

    var splitter = Container.Resolve<IByMethodsSplitter>();

    var splitContext = new SplitContext(events, string.Empty, InlineMode.EventsAndMethodsEvents, false, true);
    var methods = splitter.Split(splitContext);
    var asyncMethodsPrefix = Container.Resolve<IAsyncMethodsGrouper>().AsyncMethodsPrefix;

    var asyncMethods = methods.Where(pair => pair.Key.StartsWith(asyncMethodsPrefix));
    var sb = new StringBuilder();
    var filter = new Regex(knownSolution.NamespaceFilterPattern);

    foreach (var (methodName, methodsTraces) in asyncMethods)
    {
      if (!filter.IsMatch(methodName)) continue;

      sb.Append(methodName);

      var allocationTraces = methodsTraces.Select(allocationsExtractor).Where(t => t.Count > 0).OrderBy(t => t[0].Time.QpcStamp);

      foreach (var trace in allocationTraces)
      {
        sb.AppendNewLine().Append("Trace:").AppendNewLine();
        sb.Append(tracesDumber(trace));
      }

      sb.AppendNewLine().AppendNewLine();
    }

    return sb.ToString();
  }
}