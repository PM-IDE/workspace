using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Sockets;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ConnectStartStopFailedLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "SocketConnect",
    [TraceEventsConstants.ConnectStart],
    [TraceEventsConstants.ConnectStop, TraceEventsConstants.ConnectFailed]
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromEventActivityIdIdCreationStrategy(TraceEventsConstants.SocketActivityBasePart);
}