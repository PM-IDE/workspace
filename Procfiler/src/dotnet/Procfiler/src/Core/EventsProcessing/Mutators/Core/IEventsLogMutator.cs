using Core.Container;
using Core.Events.EventRecord;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord.EventsCollection;

namespace Procfiler.Core.EventsProcessing.Mutators.Core;

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

public interface IMultipleEventsMutator : IEventsLogMutator
{
  void Process(IEventsCollection events, IGlobalDataWithStacks context);
}

public static class EventsLogMutatorExtensions
{
  public static int GetPassOrThrow(this IEventsLogMutator mutator) =>
    mutator.GetType().GetCustomAttribute<EventMutatorAttribute>()!.Pass;
}