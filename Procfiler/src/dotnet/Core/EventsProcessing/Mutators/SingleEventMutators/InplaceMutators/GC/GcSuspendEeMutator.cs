using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class GcSuspendEeMutator : MetadataValueToNameAppenderBase
{
  public override string EventType => TraceEventsConstants.GcSuspendEeStart;
  protected override IEnumerable<MetadataKeysWithTransform> Transformations { get; }


  public GcSuspendEeMutator(IProcfilerLogger logger) : base(logger)
  {
    Transformations =
    [
      new MetadataKeysWithTransform(
        TraceEventsConstants.GcSuspendEeStartReason, GenerateNameForReason, EventClassKind.Zero)
    ];
  }


  private string GenerateNameForReason(string reason) => reason switch
  {
    "SuspendOther" => "OTHER",
    "SuspendForGC" => "GC",
    "SuspendForAppDomainShutdown" => "APP_DOMAIN_SHUTDOWN",
    "SuspendForCodePitching" => "CODE_PITCHING",
    "SuspendForShutdown" => "SHUTDOWN",
    "SuspendForDebugger" => "DEBUGGER",
    "SuspendForGCPrep" => "GC_PREP",
    "SuspendForDebuggerSweep" => "DEBUGGER_SWEEP",
    _ => MutatorsUtil.CreateUnknownEventNamePartAndLog(reason, Logger)
  };
}