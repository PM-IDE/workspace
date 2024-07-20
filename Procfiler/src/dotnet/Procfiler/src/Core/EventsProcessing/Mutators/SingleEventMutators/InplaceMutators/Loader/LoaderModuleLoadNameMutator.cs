using Core.Container;
using Core.Utils;
using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Loader;

public abstract class LoaderModuleLoadUnloadNameMutatorBase(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForModuleILFileName(TraceEventsConstants.LoaderILFileName, EventClassKind.Zero)
  };
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class LoaderModuleLoadNameMutator(IProcfilerLogger logger) : LoaderModuleLoadUnloadNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.LoaderModuleLoad;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class LoaderModuleUnloadNameMutator(IProcfilerLogger logger) : LoaderModuleLoadUnloadNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.LoaderModuleUnload;
}