using Core.Container;
using Core.Utils;
using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Contention;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ContentionLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "Contention",
    new[] { TraceEventsConstants.ContentionStart },
    new[] { TraceEventsConstants.ContentionStop }
  );