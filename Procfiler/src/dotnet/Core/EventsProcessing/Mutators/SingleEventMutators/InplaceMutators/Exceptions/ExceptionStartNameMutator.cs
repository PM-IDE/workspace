using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Exceptions;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class ExceptionStartNameMutator(IProcfilerLogger logger) : MetadataValueToNameAppenderBase(logger)
{
  public override string EventType => TraceEventsConstants.ExceptionStart;

  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; } =
  [
    MetadataKeysWithTransform.CreateForTypeLikeName(TraceEventsConstants.ExceptionType, EventClassKind.Zero)
  ];
}