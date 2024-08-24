using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.ArrayPools;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class BufferAllocatedNameMutator : MetadataValueToNameAppenderBase
{
  public override string EventType => TraceEventsConstants.BufferAllocated;
  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; }


  public BufferAllocatedNameMutator(IProcfilerLogger logger) : base(logger)
  {
    Transformations = new[]
    {
      new MetadataKeysWithTransform(
        TraceEventsConstants.BufferAllocationReason, ConvertBufferAllocationKind, EventClassKind.Zero)
    };
  }


  private string ConvertBufferAllocationKind(string kind) => kind switch
  {
    "0" => "Pooled",
    "1" => "OverMaximumSize",
    "2" => "PoolExhausted",
    _ => MutatorsUtil.CreateUnknownEventNamePartAndLog(kind, Logger)
  };
}