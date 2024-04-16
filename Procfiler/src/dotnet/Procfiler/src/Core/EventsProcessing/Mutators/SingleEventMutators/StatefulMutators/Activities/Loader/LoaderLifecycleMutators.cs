using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Loader;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class LoaderAppDomainLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "LoaderAppDomain",
    new[] { TraceEventsConstants.LoaderAppDomainLoad },
    new[] { TraceEventsConstants.LoaderAppDomainUnload }
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromAttributesIdCreationStrategy("LoaderAppDomain", new[] { TraceEventsConstants.LoaderAppDomainName });
}

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class LoaderAssemblyLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "LoaderAssembly",
    new[] { TraceEventsConstants.LoaderAssemblyLoad },
    new[] { TraceEventsConstants.LoaderAssemblyUnload }
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromAttributesIdCreationStrategy("LoaderAssembly", new[] { TraceEventsConstants.LoaderAssemblyName });
}

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class LoaderModuleLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "LoaderModule",
    new[] { TraceEventsConstants.LoaderModuleLoad },
    new[] { TraceEventsConstants.LoaderModuleUnload }
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromAttributesIdCreationStrategy("LoaderModule", new[] { TraceEventsConstants.LoaderILFileName });
}