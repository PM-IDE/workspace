using System.Reflection;
using Core.Container;
using Core.Events.EventRecord;
using Core.GlobalData;

namespace Core.EventsProcessing.Mutators.Core;

public interface IEventsLogMutator
{
  IEnumerable<EventLogMutation> Mutations { get; }
}

public interface ISingleEventMutator : IEventsLogMutator
{
  void Process(EventRecordWithMetadata eventRecord, IGlobalData context);
}

public interface ISingleEventMutatorWithState : IEventsLogMutator
{
  Type StateType { get; }

  void Process(EventRecordWithMetadata eventRecord, IGlobalData context, object mutatorState);
}

public interface ISingleEventsLifecycleMutator : ISingleEventMutatorWithState;

public static class EventsLogMutatorExtensions
{
  public static int GetPassOrThrow(this IEventsLogMutator mutator) =>
    mutator.GetType().GetCustomAttribute<EventMutatorAttribute>()!.Pass;
}