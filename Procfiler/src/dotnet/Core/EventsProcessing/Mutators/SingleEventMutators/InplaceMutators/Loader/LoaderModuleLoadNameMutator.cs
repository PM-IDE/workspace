using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Loader;

public abstract class LoaderModuleLoadUnloadNameMutatorBase(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; } =
  [
    MetadataKeysWithTransform.CreateForModuleILFileName(TraceEventsConstants.LoaderILFileName, EventClassKind.Zero)
  ];
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