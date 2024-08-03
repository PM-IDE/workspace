using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Tasks;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class AwaitContinuationScheduledMutator(IProcfilerLogger logger)
  : AttributeRenamingMutatorBase(logger, TraceEventsConstants.ContinuationId, TraceEventsConstants.TaskId)
{
  public override string EventType => TraceEventsConstants.AwaitTaskContinuationScheduledSend;
}