using Autofac;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.EventsProcessing;
using ProcfilerTests.Core;

namespace ProcfilerTests.Tests;

[TestFixture]
public class MethodsStartEndTests : GoldProcessBasedTest
{
  [TestCaseSource(nameof(DefaultContexts))]
  [TestCaseSource(nameof(OnlineSerializationContexts))]
  public void Test(ContextWithSolution dto) => DoTest(dto);


  private void DoTest(ContextWithSolution dto)
  {
    ExecuteTestWithGold(
      dto.Context, events => DumpMethodCallTree(dto.Solution.NamespaceFilterPattern, events));
  }

  private string DumpMethodCallTree(string filterPattern, CollectedEvents events)
  {
    var mainThreadEvents = TestUtil.FindEventsForMainThread(events.Events);
    var processingContext = EventsProcessingContext.DoEverything(mainThreadEvents, events.GlobalData);

    var processor = Container.Resolve<IUnitedEventsProcessor>();
    processor.ProcessFullEventLog(processingContext);
    processor.ApplyMultipleMutators(mainThreadEvents, events.GlobalData, EmptyCollections<Type>.EmptySet);

    var dump = TestsMethodCallTreeDumper.CreateDump(mainThreadEvents.Select(pair => pair.Event), filterPattern);

    return dump;
  }
}