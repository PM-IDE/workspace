using Core.Constants.TraceEvents;
using Core.Container;
using Core.Utils;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Loader;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class AssemblyLoaderLifecycleMutator(IProcfilerLogger logger) :
  EventsLifecycleMutatorBase(
    logger,
    "AssemblyLoader",
    new[] { TraceEventsConstants.AssemblyLoaderStart },
    new[] { TraceEventsConstants.AssemblyLoaderStop }
  );