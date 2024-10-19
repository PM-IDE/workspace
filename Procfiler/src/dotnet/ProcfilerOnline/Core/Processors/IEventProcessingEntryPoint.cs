using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using ProcfilerOnline.Core.Statistics;
using ProcfilerOnline.Core.Updaters;

namespace ProcfilerOnline.Core.Processors;

public interface IEventProcessingEntryPoint
{
  void Process(EventProcessingContext context);
}

[AppComponent]
public class EventProcessingEntryPoint(
  IEnumerable<ITraceEventProcessor> processors,
  IEnumerable<ISingleEventMutator> mutators,
  IStatisticsManager statisticsManager
) : IEventProcessingEntryPoint
{
  private readonly IReadOnlyList<ISingleEventMutator> myOrderedSingleMutators =
    mutators.OrderBy(mutator => mutator.GetPassOrThrow()).ToList();


  public void Process(EventProcessingContext context)
  {
    foreach (var sharedDataUpdater in processors.OfType<ISharedDataUpdater>())
    {
      sharedDataUpdater.Process(context);
    }

    foreach (var mutator in myOrderedSingleMutators)
    {
      mutator.Process(context.Event, context.SharedData);
    }

    statisticsManager.UpdateProcessedEventStatistics(context.Event);
    foreach (var processor in processors.Where(p => p is not ISharedDataUpdater))
    {
      processor.Process(context);
    }
  }
}