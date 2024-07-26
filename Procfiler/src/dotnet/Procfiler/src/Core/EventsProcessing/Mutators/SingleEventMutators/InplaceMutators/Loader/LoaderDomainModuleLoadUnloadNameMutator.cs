using Core.Constants.TraceEvents;
using Core.Container;
using Core.Utils;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Loader;

public abstract class LoaderDomainModuleLoadUnloadNameMutatorBase(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForModuleILPath(TraceEventsConstants.LoaderDomainModueFilePath, EventClassKind.Zero)
  };
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