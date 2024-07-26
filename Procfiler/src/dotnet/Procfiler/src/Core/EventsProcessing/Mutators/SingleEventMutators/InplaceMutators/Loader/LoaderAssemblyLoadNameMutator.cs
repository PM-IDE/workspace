using Core.Constants.TraceEvents;
using Core.Container;
using Core.Utils;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Loader;

public abstract class LoaderAssemblyLoadUnloadNameMutatorBase(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForAssemblyName(TraceEventsConstants.LoaderAssemblyName, EventClassKind.Zero)
  };
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class LoaderAssemblyLoadNameMutator(IProcfilerLogger logger) : LoaderAssemblyLoadUnloadNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.LoaderAssemblyLoad;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class LoaderAssemblyUnloadNameMutator(IProcfilerLogger logger) : LoaderAssemblyLoadUnloadNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.LoaderAssemblyUnload;
}