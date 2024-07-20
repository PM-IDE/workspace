using Core.Container;
using Core.Utils;
using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Exceptions;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ExceptionStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ExceptionStartStop",
    new[] { TraceEventsConstants.ExceptionStart },
    new[] { TraceEventsConstants.ExceptionStop }
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ExceptionCatchStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ExceptionCatch",
    new[] { TraceEventsConstants.ExceptionCatchStart },
    new[] { TraceEventsConstants.ExceptionCatchStop }
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ExceptionFilterStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ExceptionFilter",
    new[] { TraceEventsConstants.ExceptionFilterStart },
    new[] { TraceEventsConstants.ExceptionFilterStop }
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ExceptionFinallyStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ExceptionFinally",
    new[] { TraceEventsConstants.ExceptionFinallyStart },
    new[] { TraceEventsConstants.ExceptionFinallyStop }
  );