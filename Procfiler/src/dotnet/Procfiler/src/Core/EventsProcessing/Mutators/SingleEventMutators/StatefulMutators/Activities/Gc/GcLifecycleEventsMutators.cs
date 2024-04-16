using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Gc;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class GcFinalizersStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "Finalizers",
    new[] { TraceEventsConstants.GcFinalizersStart },
    new[] { TraceEventsConstants.GcFinalizersStop }
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class GcRestartEeLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "RestartEE",
    new[] { TraceEventsConstants.GcRestartEeStart },
    new[] { TraceEventsConstants.GcRestartEeStop }
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class GcSuspendEeStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "SuspendEE",
    new[] { TraceEventsConstants.GcSuspendEeStart },
    new[] { TraceEventsConstants.GcSuspendEeStop }
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class GcProcessLifecycleEventsMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(logger, "GC", new[] { TraceEventsConstants.GcStart }, new[] { TraceEventsConstants.GcStop })
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromAttributesIdCreationStrategy("GC", new List<string> { TraceEventsConstants.GcCount });
}