using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Loader;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class AssemblyLoaderLifecycleMutator(IProcfilerLogger logger) :
  EventsLifecycleMutatorBase(
    logger,
    "AssemblyLoader",
    [TraceEventsConstants.AssemblyLoaderStart],
    [TraceEventsConstants.AssemblyLoaderStop]
  );