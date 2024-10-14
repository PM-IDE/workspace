using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Loader;

public abstract class LoaderDomainModuleLoadUnloadNameMutatorBase(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; } =
  [
    MetadataKeysWithTransform.CreateForModuleILPath(TraceEventsConstants.LoaderDomainModueFilePath, EventClassKind.Zero)
  ];
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class LoaderDomainModuleLoadNameMutator(IProcfilerLogger logger) : LoaderDomainModuleLoadUnloadNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.LoaderDomainModuleLoad;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class LoaderDomainModuleUnloadNameMutator(IProcfilerLogger logger) : LoaderDomainModuleLoadUnloadNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.LoaderDomainModuleUnload;
}