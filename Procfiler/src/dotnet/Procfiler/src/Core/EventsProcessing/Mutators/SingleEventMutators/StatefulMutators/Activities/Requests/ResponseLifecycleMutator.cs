using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Requests;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class ResponseHeaderLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "ResponseHeaders",
    new[] { TraceEventsConstants.ResponseHeadersStart },
    new[] { TraceEventsConstants.ResponseHeadersStop }
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
    new[] { TraceEventsConstants.ResponseContentStart },
    new[] { TraceEventsConstants.ResponseContentStop }
  )
{
  protected override IIdCreationStrategy IdCreationStrategy =>
    new FromEventActivityIdIdCreationStrategy(TraceEventsConstants.HttpRequestActivityBasePart);
}