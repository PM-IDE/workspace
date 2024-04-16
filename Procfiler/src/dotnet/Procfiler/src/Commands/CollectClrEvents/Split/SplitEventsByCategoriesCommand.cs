using Procfiler.Commands.CollectClrEvents.Base;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Core;
using Procfiler.Core.EventsProcessing;
using Procfiler.Core.EventsProcessing.Mutators;
using Procfiler.Core.Serialization;
using Procfiler.Core.Serialization.Core;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Commands.CollectClrEvents.Split;

public interface ISplitEventsByCategoriesCommand : ICommandWithContext<CollectClrEventsContext>;

[CommandLineCommand]
public class SplitEventsByCategoriesCommand(
  ICommandExecutorDependantOnContext commandExecutor,
  IUnitedEventsProcessor processor,
  IUndefinedThreadsEventsMerger eventsMerger,
  IStackTraceSerializer stackTraceSerializer,
  IDelegatingEventsSerializer serializer,
  IProcfilerLogger logger
) : CollectAndSplitCommandBase<string>(logger, commandExecutor, eventsMerger, processor, serializer, stackTraceSerializer),
  ISplitEventsByCategoriesCommand
{
  public override void Execute(CollectClrEventsContext context) =>
    ExecuteSimpleSplitCommand(context, SplitEventsHelper.EventClassKeyExtractor, CollectAndSplitContext.DoNothing);

  protected override Command CreateCommandInternal() =>
    new("split-by-names", "Split the events into different files based on managed thread ID");
}