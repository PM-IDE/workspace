using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Core;
using Procfiler.Core.Collector;
using Procfiler.Utils;

namespace Procfiler.Commands.CollectClrEvents.Base;

public abstract partial class CollectCommandBase(
  IProcfilerLogger logger,
  ICommandExecutorDependantOnContext commandExecutor
) : ICommandWithContext<CollectClrEventsContext>
{
  protected readonly IProcfilerLogger Logger = logger;


  public abstract void Execute(CollectClrEventsContext context);

  protected void ExecuteCommand(CollectClrEventsContext context, Action<CollectedEvents> commandAction)
  {
    using var performanceCookie = new PerformanceCookie($"{GetType().Name}::{nameof(ExecuteCommand)}", Logger);
    ClearPathBeforeProfilingIfNeeded(context.CommonContext);

    commandExecutor.Execute(context, commandAction);
  }

  private void ClearPathBeforeProfilingIfNeeded(CollectingClrEventsCommonContext commonContext)
  {
    if (!commonContext.ClearPathBefore) return;

    PathUtils.ClearPathIfExists(commonContext.OutputPath, Logger);
  }
}