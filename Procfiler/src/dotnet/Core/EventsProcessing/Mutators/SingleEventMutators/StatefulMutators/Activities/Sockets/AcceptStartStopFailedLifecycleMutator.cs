using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Sockets;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class AcceptStartStopFailedLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "SocketAccept",
    [TraceEventsConstants.AcceptStart],
    [TraceEventsConstants.AcceptFailed, TraceEventsConstants.AcceptStop]
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromEventActivityIdIdCreationStrategy(TraceEventsConstants.SocketActivityBasePart);
}