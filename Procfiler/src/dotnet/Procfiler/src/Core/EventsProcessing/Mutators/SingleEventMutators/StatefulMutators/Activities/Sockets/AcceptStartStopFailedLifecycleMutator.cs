using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Sockets;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class AcceptStartStopFailedLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "SocketAccept",
    new[] { TraceEventsConstants.AcceptStart },
    new[] { TraceEventsConstants.AcceptFailed, TraceEventsConstants.AcceptStop }
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromEventActivityIdIdCreationStrategy(TraceEventsConstants.SocketActivityBasePart);
}