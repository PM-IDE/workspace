using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Requests;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ResponseHeaderLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ResponseHeaders",
    [TraceEventsConstants.ResponseHeadersStart],
    [TraceEventsConstants.ResponseHeadersStop]
  )
{
  protected override IIdCreationStrategy IdCreationStrategy =>
    new FromEventActivityIdIdCreationStrategy(TraceEventsConstants.HttpRequestActivityBasePart);
}

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ResponseContentLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ResponseContent",
    [TraceEventsConstants.ResponseContentStart],
    [TraceEventsConstants.ResponseContentStop]
  )
{
  protected override IIdCreationStrategy IdCreationStrategy =>
    new FromEventActivityIdIdCreationStrategy(TraceEventsConstants.HttpRequestActivityBasePart);
}