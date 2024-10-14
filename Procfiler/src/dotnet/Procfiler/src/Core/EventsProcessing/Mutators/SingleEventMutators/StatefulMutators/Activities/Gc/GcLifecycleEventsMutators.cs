using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Gc;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class GcFinalizersStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "Finalizers",
    [TraceEventsConstants.GcFinalizersStart],
    [TraceEventsConstants.GcFinalizersStop]
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class GcRestartEeLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "RestartEE",
    [TraceEventsConstants.GcRestartEeStart],
    [TraceEventsConstants.GcRestartEeStop]
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class GcSuspendEeStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "SuspendEE",
    [TraceEventsConstants.GcSuspendEeStart],
    [TraceEventsConstants.GcSuspendEeStop]
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class GcProcessLifecycleEventsMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(logger, "GC", [TraceEventsConstants.GcStart], [TraceEventsConstants.GcStop])
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromAttributesIdCreationStrategy("GC", new List<string> { TraceEventsConstants.GcCount });
}