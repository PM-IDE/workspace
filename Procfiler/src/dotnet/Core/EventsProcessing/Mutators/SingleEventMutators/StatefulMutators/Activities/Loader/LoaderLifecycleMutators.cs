using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Loader;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class LoaderAppDomainLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "LoaderAppDomain",
    [TraceEventsConstants.LoaderAppDomainLoad],
    [TraceEventsConstants.LoaderAppDomainUnload]
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromAttributesIdCreationStrategy("LoaderAppDomain", [TraceEventsConstants.LoaderAppDomainName]);
}

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class LoaderAssemblyLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "LoaderAssembly",
    [TraceEventsConstants.LoaderAssemblyLoad],
    [TraceEventsConstants.LoaderAssemblyUnload]
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromAttributesIdCreationStrategy("LoaderAssembly", [TraceEventsConstants.LoaderAssemblyName]);
}

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class LoaderModuleLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    "LoaderModule",
    [TraceEventsConstants.LoaderModuleLoad],
    [TraceEventsConstants.LoaderModuleUnload]
  )
{
  protected override IIdCreationStrategy IdCreationStrategy { get; } =
    new FromAttributesIdCreationStrategy("LoaderModule", [TraceEventsConstants.LoaderILFileName]);
}