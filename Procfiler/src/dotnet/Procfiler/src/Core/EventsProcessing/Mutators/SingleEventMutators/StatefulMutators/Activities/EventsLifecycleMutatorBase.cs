using Core.Constants.XesLifecycle;
using Core.Events.EventRecord;
using Core.Exceptions;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.EventsProcessing.Mutators.Core;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities;

public enum LifecycleState
{
  Unknown,
  Schedule,
  Start,
  Complete
}

public record EventsLifecycleMutatorState
{
  public record State
  {
    public LifecycleState LifecycleState { get; set; } = LifecycleState.Unknown;
    public string? CurrentId { get; set; }
  }


  public Dictionary<string, State> StatesByActivities { get; } = new();
}

public abstract class EventsLifecycleMutatorBase : ISingleEventsLifecycleMutator
{
  private readonly IProcfilerLogger myLogger;
  private readonly string? myScheduleEventClass;
  private readonly HashSet<string> myStartEventClasses;
  private readonly HashSet<string> myCompleteEventClasses;
  private readonly HashSet<string> myAllProcessableEvents;


  protected virtual IIdCreationStrategy IdCreationStrategy { get; }


  public Type StateType => typeof(EventsLifecycleMutatorState);


  protected EventsLifecycleMutatorBase(
    IProcfilerLogger logger,
    string activityName,
    ICollection<string> startEventClasses,
    ICollection<string> completeEventClasses,
    string? scheduleEventClass = null)
  {
    myLogger = logger;
    myStartEventClasses = startEventClasses.ToHashSet();
    myCompleteEventClasses = completeEventClasses.ToHashSet();
    myScheduleEventClass = scheduleEventClass;

    myAllProcessableEvents = [];
    myAllProcessableEvents.UnionWith(myStartEventClasses);
    myAllProcessableEvents.UnionWith(myCompleteEventClasses);

    if (myScheduleEventClass is { })
    {
      myAllProcessableEvents.Add(myScheduleEventClass);
    }

    IdCreationStrategy = new DefaultIdCreationStrategy(activityName, myStartEventClasses);
  }


  public IEnumerable<EventLogMutation> Mutations
  {
    get
    {
      var mutations = new List<EventLogMutation>();
      foreach (var startEventClass in myStartEventClasses)
      {
        mutations.Add(new AddLifecycleTransitionAttributeMutation(startEventClass, XesStandardLifecycleConstants.Start));
      }

      if (myScheduleEventClass is { })
      {
        mutations.Add(new AddLifecycleTransitionAttributeMutation(myScheduleEventClass, XesStandardLifecycleConstants.Schedule));
      }

      foreach (var eventClass in myCompleteEventClasses)
      {
        var mutation = new AddLifecycleTransitionAttributeMutation(eventClass, XesStandardLifecycleConstants.Complete);
        mutations.Add(mutation);
      }

      foreach (var eventClass in myAllProcessableEvents)
      {
        mutations.Add(new ActivityIdCreation(eventClass, IdCreationStrategy.CreateIdTemplate()));
      }

      return mutations;
    }
  }

  public void Process(
    EventRecordWithMetadata eventRecord, IGlobalData context, object mutatorState)
  {
    //todo: to state machine with "Stateless" nuget
    if (!myAllProcessableEvents.Contains(eventRecord.EventClass)) return;

    if (mutatorState is not EventsLifecycleMutatorState statesByActivities)
    {
      throw new NotExpectedStateException(StateType, mutatorState.GetType());
    }

    var activityId = IdCreationStrategy.CreateId(eventRecord);
    if (!statesByActivities.StatesByActivities.TryGetValue(activityId, out var state))
    {
      state = new EventsLifecycleMutatorState.State();
      statesByActivities.StatesByActivities[activityId] = state;
    }

    state.CurrentId = activityId;
    if (myScheduleEventClass is { } && eventRecord.EventClass == myScheduleEventClass)
    {
      if (state.LifecycleState != LifecycleState.Unknown)
      {
        myLogger.LogTrace(
          $"Scheduling activity in with state {state.LifecycleState} for {eventRecord.EventClass} with id {activityId} on {eventRecord.ManagedThreadId}");

        return;
      }

      state.LifecycleState = LifecycleState.Schedule;
      state.CurrentId = activityId;
      StandardLifecycleModelUtil.MarkAsScheduled(eventRecord, state.CurrentId);
    }
    else if (myStartEventClasses.Contains(eventRecord.EventClass))
    {
      if (state.LifecycleState != LifecycleState.Schedule && state.LifecycleState != LifecycleState.Unknown)
      {
        myLogger.LogTrace(
          $"Starting activity from incorrect state {state.LifecycleState} for {eventRecord.EventClass} with id {activityId} on {eventRecord.ManagedThreadId}");

        return;
      }

      Debug.Assert(state.CurrentId is { });
      state.LifecycleState = LifecycleState.Start;
      StandardLifecycleModelUtil.MarkAsStarted(eventRecord, state.CurrentId);
    }
    else if (myCompleteEventClasses.Contains(eventRecord.EventClass))
    {
      if (state.LifecycleState != LifecycleState.Start)
      {
        myLogger.LogTrace(
          $"Met completed activity without start {state.LifecycleState} for {eventRecord.EventClass} with id {activityId} on {eventRecord.ManagedThreadId}");

        return;
      }

      Debug.Assert(state.CurrentId is { });
      StandardLifecycleModelUtil.MarkAsCompleted(eventRecord, state.CurrentId);

      state.LifecycleState = LifecycleState.Unknown;
      state.CurrentId = null;
    }
  }
}