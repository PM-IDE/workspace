using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Methods;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class MethodR2REventLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "R2REntryPoint",
    [TraceEventsConstants.MethodR2RGetEntryPointStart],
    [TraceEventsConstants.MethodR2RGetEntryPoint]
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class MethodLoadUnloadLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "MethodLoading",
    [TraceEventsConstants.MethodLoadVerbose],
    [TraceEventsConstants.MethodUnloadVerbose]
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } = new FromAttributesIdCreationStrategy("MethodLoadUnload", [
    TraceEventsConstants.MethodNamespace,
    TraceEventsConstants.MethodName,
    TraceEventsConstants.MethodSignature
  ]);
}