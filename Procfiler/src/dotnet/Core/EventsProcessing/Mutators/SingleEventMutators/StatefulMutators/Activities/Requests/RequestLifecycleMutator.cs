using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Requests;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class RequestStartStopLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "Request",
    [TraceEventsConstants.RequestStart],
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
    [TraceEventsConstants.RequestContentStart],
    [TraceEventsConstants.RequestContentStop]
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
    [TraceEventsConstants.RequestHeadersStart],
    [TraceEventsConstants.RequestHeadersStop]
  )
{
  protected override IIdCreationStrategy IdCreationStrategy =>
    new FromEventActivityIdIdCreationStrategy(TraceEventsConstants.HttpRequestActivityBasePart);
}