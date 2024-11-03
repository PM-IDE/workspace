using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Contention;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ContentionLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "Contention",
    [TraceEventsConstants.ContentionStart],
    [TraceEventsConstants.ContentionStop]
  );