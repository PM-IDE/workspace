using Core.Container;
using Core.Utils;
using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Loader;

public abstract class LoaderAppDomainLoadUnloadNameMutatorBase(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForCamelCaseName(TraceEventsConstants.LoaderAppDomainName, EventClassKind.Zero)
  };
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