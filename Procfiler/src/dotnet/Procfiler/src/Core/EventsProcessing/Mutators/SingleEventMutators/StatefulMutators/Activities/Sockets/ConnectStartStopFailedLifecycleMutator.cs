using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Sockets;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ConnectStartStopFailedLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "SocketConnect",
    new[] { TraceEventsConstants.ConnectStart },
    new[] { TraceEventsConstants.ConnectStop, TraceEventsConstants.ConnectFailed }
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromEventActivityIdIdCreationStrategy(TraceEventsConstants.SocketActivityBasePart);
}