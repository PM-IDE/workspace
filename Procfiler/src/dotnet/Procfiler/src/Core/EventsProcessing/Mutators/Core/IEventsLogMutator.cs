using Core.Container;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection;

namespace Procfiler.Core.EventsProcessing.Mutators.Core;

public interface IEventsLogMutator
{
  IEnumerable<EventLogMutation> Mutations { get; }
}

public interface ISingleEventMutator : IEventsLogMutator
{
  void Process(EventRecordWithMetadata eventRecord, SessionGlobalData context);
}

public interface ISingleEventMutatorWithState : IEventsLogMutator
{
  Type StateType { get; }

  void Process(EventRecordWithMetadata eventRecord, SessionGlobalData context, object mutatorState);
}

public interface ISingleEventsLifecycleMutator : ISingleEventMutatorWithState;

public interface IMultipleEventsMutator : IEventsLogMutator
{
  void Process(IEventsCollection events, SessionGlobalData context);
}

public static class EventsLogMutatorExtensions
{
  public static int GetPassOrThrow(this IEventsLogMutator mutator) =>
    mutator.GetType().GetCustomAttribute<EventMutatorAttribute>()!.Pass;
}