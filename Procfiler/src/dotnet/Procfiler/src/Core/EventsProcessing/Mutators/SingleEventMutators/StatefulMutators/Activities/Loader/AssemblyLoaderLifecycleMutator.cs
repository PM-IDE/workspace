using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Loader;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class AssemblyLoaderLifecycleMutator(IProcfilerLogger logger) :
  EventsLifecycleMutatorBase(
    logger,
    "AssemblyLoader",
    new[] { TraceEventsConstants.AssemblyLoaderStart },
    new[] { TraceEventsConstants.AssemblyLoaderStop }
  );