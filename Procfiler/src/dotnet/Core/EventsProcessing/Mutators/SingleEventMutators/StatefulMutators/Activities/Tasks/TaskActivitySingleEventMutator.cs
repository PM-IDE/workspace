using Core.Constants.TraceEvents;
using Core.Container;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Tasks;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class TaskActivitySingleEventMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    ActivityId,
    [TraceEventsConstants.TaskExecuteStart],
    [TraceEventsConstants.TaskExecuteStop],
    TraceEventsConstants.TaskScheduledSend
  )
{
  private const string ActivityId = "TaskExecute";


  protected override IIdCreationStrategy IdCreationStrategy { get; } = new FromAttributesIdCreationStrategy(ActivityId, [
    TraceEventsConstants.TaskId
  ]);
}

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class TaskWaitBeginLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(logger, ActivityId, [TraceEventsConstants.TaskWaitSend],
    [TraceEventsConstants.TaskWaitStop])
{
  private const string ActivityId = "TaskWaitBeginEnd";

  protected override IIdCreationStrategy IdCreationStrategy { get; } = new FromAttributesIdCreationStrategy(ActivityId, [
    TraceEventsConstants.TaskId
  ]);
}

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class TaskContinuationWaitLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(logger, ActivityId, ourStartEventClasses, [TraceEventsConstants.TaskWaitContinuationComplete])
{
  private static readonly HashSet<string> ourStartEventClasses =
  [
    TraceEventsConstants.TaskWaitContinuationStarted,
    TraceEventsConstants.AwaitTaskContinuationScheduledSend
  ];


  private const string ActivityId = "TaskContinuationWait";


  protected override IIdCreationStrategy IdCreationStrategy { get; } = new FromAttributesIdCreationStrategy(ActivityId, [
    TraceEventsConstants.TaskId
  ]);
}