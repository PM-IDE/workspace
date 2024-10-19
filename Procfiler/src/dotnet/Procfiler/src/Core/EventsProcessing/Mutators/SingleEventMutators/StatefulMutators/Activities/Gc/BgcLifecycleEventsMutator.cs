using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Gc;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class InitialBlockingMarkingLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "InitialBlockingMarking",
    [TraceEventsConstants.BgcStart],
    [TraceEventsConstants.Bgc1StNonCondStop]
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class FinalBlockingMarkingLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "FinalBlockingMarking",
    [TraceEventsConstants.Bgc2NdNonConStart],
    [TraceEventsConstants.Bgc2NdNonConStop]
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ConcurrentSweepLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ConcurrentSweep",
    [TraceEventsConstants.Bgc2NdConStart],
    [TraceEventsConstants.Bgc2NdConStop]
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class LohAllocationsSuppressionLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "LOHAllocationsSuppression",
    [TraceEventsConstants.BgcAllocWaitStart],
    [TraceEventsConstants.BgcAllocWaitStop]
  );