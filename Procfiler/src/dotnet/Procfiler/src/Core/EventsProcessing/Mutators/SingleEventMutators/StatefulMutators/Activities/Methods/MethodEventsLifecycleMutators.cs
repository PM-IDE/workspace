using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Methods;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class MethodR2REventLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "R2REntryPoint",
    new[] { TraceEventsConstants.MethodR2RGetEntryPointStart },
    new[] { TraceEventsConstants.MethodR2RGetEntryPoint }
  );

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class MethodLoadUnloadLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "MethodLoading",
    new[] { TraceEventsConstants.MethodLoadVerbose },
    new[] { TraceEventsConstants.MethodUnloadVerbose }
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } = new FromAttributesIdCreationStrategy("MethodLoadUnload", new[]
  {
    TraceEventsConstants.MethodNamespace,
    TraceEventsConstants.MethodName,
    TraceEventsConstants.MethodSignature
  });
}