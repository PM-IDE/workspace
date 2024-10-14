using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Exceptions;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ExceptionStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ExceptionStartStop",
    [TraceEventsConstants.ExceptionStart],
    [TraceEventsConstants.ExceptionStop]
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ExceptionCatchStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ExceptionCatch",
    [TraceEventsConstants.ExceptionCatchStart],
    [TraceEventsConstants.ExceptionCatchStop]
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ExceptionFilterStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ExceptionFilter",
    [TraceEventsConstants.ExceptionFilterStart],
    [TraceEventsConstants.ExceptionFilterStop]
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ExceptionFinallyStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ExceptionFinally",
    [TraceEventsConstants.ExceptionFinallyStart],
    [TraceEventsConstants.ExceptionFinallyStop]
  );