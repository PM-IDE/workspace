using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.AssemblyLoader;

public abstract class AssemblyLoaderStartStopNameMutatorBase(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForAssemblyName(TraceEventsConstants.AssemblyName, EventClassKind.Zero)
  };
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class AssemblyLoaderStartNameMutator(IProcfilerLogger logger) : AssemblyLoaderStartStopNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.AssemblyLoaderStart;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class AssemblyLoaderStopNameMutator(IProcfilerLogger logger) : AssemblyLoaderStartStopNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.AssemblyLoaderStop;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class AssemblyLoaderKnownPathProbedNameMutator(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForModuleILPath(TraceEventsConstants.FilePath, EventClassKind.Zero)
  };

  public override string EventType => TraceEventsConstants.AssemblyLoaderKnownPathProbed;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class AssemblyLoaderResolutionAttemptedNameMutator(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForAssemblyName(TraceEventsConstants.AssemblyName, EventClassKind.Zero),
    MetadataKeysWithTransform.CreateIdenticalTransform(TraceEventsConstants.Stage, EventClassKind.NotEventClass),
    MetadataKeysWithTransform.CreateIdenticalTransform(TraceEventsConstants.Result, EventClassKind.NotEventClass)
  };


  public override string EventType => TraceEventsConstants.AssemblyLoaderResolutionAttempted;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class AssemblyLoaderAppDomainAssemblyResolveHandlerInvokeNameMutator(
  IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForAssemblyName(TraceEventsConstants.AssemblyName, EventClassKind.Zero),
    MetadataKeysWithTransform.CreateIdenticalTransform(TraceEventsConstants.HandlerName, EventClassKind.Zero)
  };


  public override string EventType => TraceEventsConstants.AssemblyLoaderAppDomainAssemblyResolveHandlerInvoked;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class AssemblyLoaderLoadContextResolvingHandlerInvokedNameMutator(
  IProcfilerLogger logger,
  bool removeProperties = false) : MetadataValueToNameAppenderBase(logger, removeProperties)
{
  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; } = new[]
  {
    MetadataKeysWithTransform.CreateForAssemblyName(TraceEventsConstants.AssemblyName, EventClassKind.Zero),
    MetadataKeysWithTransform.CreateIdenticalTransform(TraceEventsConstants.HandlerName, EventClassKind.Zero),
    MetadataKeysWithTransform.CreateIdenticalTransform(TraceEventsConstants.AssemblyLoadContext, EventClassKind.Zero)
  };


  public override string EventType => TraceEventsConstants.AssemblyLoaderAssemblyLoadFromResolveHandlerInvoked;
}