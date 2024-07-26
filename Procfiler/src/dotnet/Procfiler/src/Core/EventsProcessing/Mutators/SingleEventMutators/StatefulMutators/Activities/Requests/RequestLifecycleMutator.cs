using Core.Constants.TraceEvents;
using Core.Container;
using Core.Utils;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Requests;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class RequestStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "Request",
    new[] { TraceEventsConstants.RequestStart },
    ourCompleteEvents,
    TraceEventsConstants.RequestLeftQueue
  )
{
  private static readonly string[] ourCompleteEvents =
  [
    TraceEventsConstants.RequestStop, TraceEventsConstants.RequestFailed
  ];


  protected override IIdCreationStrategy IdCreationStrategy =>
    new FromEventActivityIdIdCreationStrategy(TraceEventsConstants.HttpRequestActivityBasePart);
}

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class RequestContentLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "RequestContent",
    new[] { TraceEventsConstants.RequestContentStart },
    new[] { TraceEventsConstants.RequestContentStop }
  )
{
  protected override IIdCreationStrategy IdCreationStrategy =>
    new FromEventActivityIdIdCreationStrategy(TraceEventsConstants.HttpRequestActivityBasePart);
}

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class RequestHeaderLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "RequestHeaders",
    new[] { TraceEventsConstants.RequestHeadersStart },
    new[] { TraceEventsConstants.RequestHeadersStop }
  )
{
  protected override IIdCreationStrategy IdCreationStrategy =>
    new FromEventActivityIdIdCreationStrategy(TraceEventsConstants.HttpRequestActivityBasePart);
}