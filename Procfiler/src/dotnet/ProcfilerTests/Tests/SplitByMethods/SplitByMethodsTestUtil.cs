using Autofac;
using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.Collector;
using Procfiler.Core.EventsProcessing;
using Procfiler.Core.SplitByMethod;
using ProcfilerTests.Core;

namespace ProcfilerTests.Tests.SplitByMethods;

public static class SplitByMethodsTestUtil
{
  public static IReadOnlyDictionary<string, List<List<EventRecordWithMetadata>>> SplitByMethods(
    CollectedEvents events, IContainer container, string filterPattern)
  {
    var mainThreadEvents = TestUtil.FindEventsForMainThread(events.Events);
    var processingContext = EventsProcessingContext.DoEverything(mainThreadEvents, events.GlobalData);

    var processor = container.Resolve<IUnitedEventsProcessor>();
    processor.ProcessFullEventLog(processingContext);
    processor.ApplyMultipleMutators(mainThreadEvents, events.GlobalData, EmptyCollections<Type>.EmptySet);

    var splitter = container.Resolve<IEventsCollectionByMethodsSplitter>();
    return splitter.Split(mainThreadEvents, filterPattern, InlineMode.EventsAndMethodsEvents);
  }
}