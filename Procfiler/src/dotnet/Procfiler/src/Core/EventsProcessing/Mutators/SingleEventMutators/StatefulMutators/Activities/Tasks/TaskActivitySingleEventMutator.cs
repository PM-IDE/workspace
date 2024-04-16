using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Core.EventsProcessing.Mutators.Core.Passes;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators.Activities.Tasks;

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class TaskActivitySingleEventMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(
    logger,
    ActivityId,
    new[] { TraceEventsConstants.TaskExecuteStart },
    new[] { TraceEventsConstants.TaskExecuteStop },
    TraceEventsConstants.TaskScheduledSend
  )
{
  private const string ActivityId = "TaskExecute";


  protected override IIdCreationStrategy IdCreationStrategy { get; } = new FromAttributesIdCreationStrategy(ActivityId, new[]
  {
    TraceEventsConstants.TaskId
  });
}

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class TaskWaitBeginLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(logger, ActivityId, new[] { TraceEventsConstants.TaskWaitSend }, new[] { TraceEventsConstants.TaskWaitStop })
{
  private const string ActivityId = "TaskWaitBeginEnd";

  protected override IIdCreationStrategy IdCreationStrategy { get; } = new FromAttributesIdCreationStrategy(ActivityId, new[]
  {
    TraceEventsConstants.TaskId
  });
}

[EventMutator(SingleEventMutatorsPasses.ActivityAttributesSetter)]
public class TaskContinuationWaitLifecycleMutator(IProcfilerLogger logger)
  : EventsLifecycleMutatorBase(logger, ActivityId, ourStartEventClasses, new[] { TraceEventsConstants.TaskWaitContinuationComplete })
{
  private static readonly HashSet<string> ourStartEventClasses = new()
  {
    TraceEventsConstants.TaskWaitContinuationStarted,
    TraceEventsConstants.AwaitTaskContinuationScheduledSend
  };

  private const string ActivityId = "TaskContinuationWait";


  protected override IIdCreationStrategy IdCreationStrategy { get; } = new FromAttributesIdCreationStrategy(ActivityId, new[]
  {
    TraceEventsConstants.TaskId
  });
}