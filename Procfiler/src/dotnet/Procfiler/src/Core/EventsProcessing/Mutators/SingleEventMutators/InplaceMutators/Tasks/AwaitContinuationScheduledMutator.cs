using Core.Constants.TraceEvents;
using Core.Container;
using Core.Utils;
using Procfiler.Core.EventsProcessing.Mutators.Core;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.Tasks;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class AwaitContinuationScheduledMutator(IProcfilerLogger logger)
  : AttributeRenamingMutatorBase(logger, TraceEventsConstants.ContinueWithTaskId, TraceEventsConstants.TaskId)
{
  public override string EventType => TraceEventsConstants.AwaitTaskContinuationScheduledSend;
}