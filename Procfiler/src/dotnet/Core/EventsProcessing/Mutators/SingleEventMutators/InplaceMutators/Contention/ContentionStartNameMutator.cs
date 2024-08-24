using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Contention;

public abstract class ContentionStartStopNameMutatorBase : MetadataValueToNameAppenderBase
{
  protected sealed override IEnumerable<MetadataKeysWithTransform> Transformations { get; }


  protected ContentionStartStopNameMutatorBase(IProcfilerLogger logger) : base(logger)
  {
    Transformations = new[]
    {
      new MetadataKeysWithTransform(TraceEventsConstants.ContentionFlags, TransformContentionKind, EventClassKind.Zero)
    };
  }


  private string TransformContentionKind(string kind) => kind switch
  {
    "Managed" => "MANAGED",
    "Native" => "NATIVE",
    _ => MutatorsUtil.CreateUnknownEventNamePartAndLog(kind, Logger)
  };
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class ContentionStartNameMutator(IProcfilerLogger logger) : ContentionStartStopNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.ContentionStart;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class ContentionEndNameMutator(IProcfilerLogger logger) : ContentionStartStopNameMutatorBase(logger)
{
  public override string EventType => TraceEventsConstants.ContentionStop;
}