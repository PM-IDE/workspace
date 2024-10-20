using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Loader;

public abstract class LoaderAppDomainLoadUnloadNameMutatorBase(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; } =
  [
    MetadataKeysWithTransform.CreateForCamelCaseName(TraceEventsConstants.LoaderAppDomainName, EventClassKind.Zero)
  ];
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class LoaderAppDomainLoadNameMutator(IProcfilerLogger logger) : LoaderAppDomainLoadUnloadNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.LoaderAppDomainLoad;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class LoaderAppDomainUnloadNameMutator(IProcfilerLogger logger) : LoaderAppDomainLoadUnloadNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.LoaderAppDomainUnload;
}