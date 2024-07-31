using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Contention;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ContentionLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "Contention",
    new[] { TraceEventsConstants.ContentionStart },
    new[] { TraceEventsConstants.ContentionStop }
  );