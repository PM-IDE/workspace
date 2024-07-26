using Core.Container;
using Core.Events.EventsCollection;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.EventsProcessing.Filters.Core;
using Procfiler.Core.EventsProcessing.Mutators;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.MultipleEventsMutators;

namespace Procfiler.Core.EventsProcessing;

public readonly record struct EventsProcessingConfig(
  bool UseFilters,
  bool UseMutators,
  ISet<Type> DisabledMutators)
{
  public static EventsProcessingConfig DoEverything { get; } = new(true, true, EmptyCollections<Type>.EmptySet);

  public static EventsProcessingConfig EverythingWithoutMethodStartAndEnd { get; } =
    new(true, true, new HashSet<Type> { typeof(MethodStartEndEventsLogMutator) });

  public static EventsProcessingConfig CreateContextWithoutStartAndEnd(bool useFilters, bool useMutators) =>
    new(
      useFilters, useMutators, new HashSet<Type> { typeof(MethodStartEndEventsLogMutator) });
}

public readonly record struct EventsProcessingContext(
  IEventsCollection ThreadEvents,
  SessionGlobalData SessionGlobalData,
  EventsProcessingConfig Config
)
{
  public static EventsProcessingContext DoEverything(IEventsCollection events, SessionGlobalData globalData)
  {
    var config = EventsProcessingConfig.DoEverything;
    return new EventsProcessingContext(events, globalData, config);
  }

  public static EventsProcessingContext DoEverythingWithoutMethodStartEnd(IEventsCollection events, SessionGlobalData globalData)
  {
    var config = EventsProcessingConfig.EverythingWithoutMethodStartAndEnd;
    return new EventsProcessingContext(events, globalData, config);
  }
}

public interface IUnitedEventsProcessor
{
  void ProcessFullEventLog(in EventsProcessingContext context);
  void ApplyMultipleMutators(IEventsCollection events, SessionGlobalData globalData, ISet<Type> disabledMutators);
}

[AppComponent]
public class UnitedEventsProcessorImpl : IUnitedEventsProcessor
{
  private readonly IEnumerable<ISingleEventMutator> mySingleEventMutators;
  private readonly IEnumerable<IMultipleEventsMutator> myMultipleEventsMutators;
  private readonly IEnumerable<ISingleEventMutatorWithState> myStatefulSingleEventsMutators;
  private readonly IEventsFilterer myFilterer;
  private readonly IProcfilerLogger myLogger;


  public UnitedEventsProcessorImpl(
    IProcfilerLogger logger,
    IEnumerable<ISingleEventMutator> singleEventMutators,
    IEnumerable<IMultipleEventsMutator> multipleEventsMutators,
    IEnumerable<ISingleEventMutatorWithState> statefulSingleEventsMutators,
    IUndefinedThreadsEventsMerger undefinedThreadsEventsMerger,
    IEventsFilterer filterer)
  {
    mySingleEventMutators = singleEventMutators.OrderBy(mutator => mutator.GetPassOrThrow()).ToList();
    myMultipleEventsMutators = multipleEventsMutators.OrderBy(mutator => mutator.GetPassOrThrow()).ToList();
    myLogger = logger;
    myStatefulSingleEventsMutators = statefulSingleEventsMutators;
    myFilterer = filterer;
  }


  public void ApplyMultipleMutators(IEventsCollection events, SessionGlobalData globalData, ISet<Type> disabledMutators)
  {
    using var __ = new PerformanceCookie($"{GetType().Name}::{nameof(ApplyMultipleMutators)}::Mutators", myLogger);
    var multipleMutators = myMultipleEventsMutators.Where(m => !disabledMutators.Contains(m.GetType())).ToList();

    foreach (var multipleEventsMutator in multipleMutators)
    {
      multipleEventsMutator.Process(events, globalData);
    }
  }

  public void ProcessFullEventLog(in EventsProcessingContext context)
  {
    var (events, globalData, (useFilters, useMutators, disabledMutators)) = context;

    if (useFilters)
    {
      ApplyFilters(events);
    }

    if (useMutators)
    {
      ApplySingleMutators(events, globalData, disabledMutators);
    }
  }

  private void ApplySingleMutators(IEventsCollection events, SessionGlobalData globalData, ISet<Type> disabledMutators)
  {
    var states = CreateMutatorsStates(myStatefulSingleEventsMutators);
    var singleMutators = mySingleEventMutators.Where(m => !disabledMutators.Contains(m.GetType())).ToList();
    var statefulMutators = myStatefulSingleEventsMutators.Where(m => !disabledMutators.Contains(m.GetType())).ToList();

    foreach (var (_, eventRecord) in events)
    {
      foreach (var singleEventMutator in singleMutators)
      {
        singleEventMutator.Process(eventRecord, globalData);
      }

      foreach (var (mutatorWithState, state) in statefulMutators.Zip(states))
      {
        mutatorWithState.Process(eventRecord, globalData, state);
      }
    }
  }

  private void ApplyFilters(IEventsCollection events)
  {
    using var __ = new PerformanceCookie($"{GetType().Name}::{nameof(ApplyMultipleMutators)}::Filters", myLogger);
    myFilterer.Filter(events);
  }

  private static object[] CreateMutatorsStates(IEnumerable<ISingleEventMutatorWithState> mutators)
  {
    return mutators
      .Select(m => Activator.CreateInstance(m.StateType) ?? throw new Exception())
      .ToArray();
  }
}