using Core.Builder;
using Core.Container;
using Core.CppProcfiler;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Core.Collector;
using Procfiler.Core.CppProcfiler;
using Procfiler.Core.Processes;

namespace Procfiler.Commands.CollectClrEvents;

public interface ICommandExecutorDependantOnContext
{
  void Execute(CollectClrEventsContext context, Action<CollectedEvents> commandAction);
}

[AppComponent]
public class CommandExecutorImpl(
  IDotnetProcessLauncher dotnetProcessLauncher,
  IClrEventsCollector clrEventsCollector,
  IDotnetProjectBuilder projectBuilder,
  IProcfilerLogger logger,
  ICppProcfilerLocator cppProcfilerLocator,
  IBinaryStackSavePathCreator binaryStackSavePathCreator
) : ICommandExecutorDependantOnContext
{
  public void Execute(CollectClrEventsContext context, Action<CollectedEvents> commandAction)
  {
    switch (context)
    {
      case CollectClrEventsFromExeWithRepeatContext repeatContext:
        ExecuteCommandWithRetryExe(repeatContext, commandAction);
        return;
      case CollectClrEventsFromExeWithArguments argsContext:
        ExecuteCommandWithArgumentsList(argsContext, commandAction);
        return;
      case CollectClrEventsFromExeContext exeContext:
        ExecuteCommandWithLaunchingProcess(exeContext, commandAction);
        return;
      case CollectClrEventsFromRunningProcessContext runContext:
        ExecuteCommandWithRunningProcess(runContext, commandAction);
        return;
      case CollectClrEventsFromCommandContext commandContext:
        ExecuteSpecifiedCommand(commandContext, commandAction);
        return;
      default:
        throw new ArgumentOutOfRangeException(nameof(context), context, null);
    }
  }

  private void ExecuteSpecifiedCommand(CollectClrEventsFromCommandContext context, Action<CollectedEvents> commandAction)
  {
    if (context.Arguments is { })
    {
      foreach (var currentArguments in context.Arguments)
      {
        var newContext = context with
        {
          CommonContext = context.CommonContext with
          {
            Arguments = currentArguments
          }
        };

        DoExecuteCommand(newContext, commandAction);
      }

      return;
    }

    DoExecuteCommand(context, commandAction);
  }

  private void DoExecuteCommand(CollectClrEventsFromCommandContext context, Action<CollectedEvents> commandAction)
  {
    var laucnherDto = DotnetProcessLauncherDto.CreateFrom(
      context.CommonContext, context.CommandName, cppProcfilerLocator, binaryStackSavePathCreator);

    ExecuteDotnetProcess(context, laucnherDto, context.CommandName, commandAction);
  }

  private void ExecuteCommandWithArgumentsList(
    CollectClrEventsFromExeWithArguments context, Action<CollectedEvents> commandAction)
  {
    foreach (var currentArguments in context.Arguments)
    {
      var newContext = context with
      {
        CommonContext = context.CommonContext with
        {
          Arguments = currentArguments
        }
      };

      ExecuteCommandWithLaunchingProcess(newContext, commandAction);
    }
  }

  private void ExecuteCommandWithRetryExe(
    CollectClrEventsFromExeWithRepeatContext context, Action<CollectedEvents> commandAction)
  {
    for (var i = 0; i < context.RepeatCount; i++)
    {
      ExecuteCommandWithLaunchingProcess(context, commandAction);
    }
  }

  private void ExecuteCommandWithRunningProcess(
    CollectClrEventsFromRunningProcessContext context,
    Action<CollectedEvents> commandAction)
  {
    if (CollectEventsFromProcess(context, context.ProcessId, null) is var events)
    {
      commandAction(events);
    }
  }

  private CollectedEvents CollectEventsFromProcess(
    CollectClrEventsContext context, int processId, string? binStacksPath) =>
    clrEventsCollector.CollectEvents(ToCollectionContext(context, processId, binStacksPath));

  private ClrEventsCollectionContext ToCollectionContext(CollectClrEventsContext context, int processId, string? binStacksPath)
  {
    var ctx = context.CommonContext;
    var (_, _, _, _, category, _, duration, timeout, _, _, _, cppProfilerMode, _, _, _, _, _) = ctx;

    if (ctx.CppProfilerMode.IsDisabled())
    {
      return new FromEventsStacksClrEventsCollectionContext(processId, duration, timeout, category);
    }

    if (binStacksPath is not { })
    {
      logger.LogError("Bin stacks path was null when cpp profiler is enabled");
      throw new ArgumentNullException(nameof(binStacksPath));
    }

    return new BinaryStacksClrEventsCollectionContext(processId, duration, timeout, category, cppProfilerMode, binStacksPath);
  }

  private void ExecuteCommandWithLaunchingProcess(
    CollectClrEventsFromExeContext context,
    Action<CollectedEvents> commandAction)
  {
    var (pathToCsproj, tfm, _, _, _, _, _, _) = context.ProjectBuildInfo;
    var buildResultNullable = projectBuilder.TryBuildDotnetProject(context.ProjectBuildInfo);
    if (!buildResultNullable.HasValue)
    {
      logger.LogError("Failed to build dotnet project {Tfm}, {Path}", tfm, pathToCsproj);
      return;
    }

    var buildResult = buildResultNullable.Value;
    var launcherDto = DotnetProcessLauncherDto.CreateFrom(
      context.CommonContext, buildResult, cppProcfilerLocator, binaryStackSavePathCreator);

    try
    {
      ExecuteDotnetProcess(context, launcherDto, buildResult.BuiltDllPath, commandAction);
    }
    finally
    {
      if (context.CommonContext.ClearArtifacts)
      {
        buildResult.ClearUnderlyingFolder();
      }
    }
  }

  private void ExecuteDotnetProcess(
    CollectClrEventsContext context,
    DotnetProcessLauncherDto launcherDto,
    string commandName,
    Action<CollectedEvents> commandAction)
  {
    try
    {
      if (dotnetProcessLauncher.TryStartDotnetProcess(launcherDto) is not { } process)
      {
        logger.LogError("Failed to start or to find process");
        return;
      }

      var sb = new StringBuilder();
      if (context.CommonContext.PrintProcessOutput)
      {
        process.OutputDataReceived += (_, args) => { sb.Append(args.Data); };

        process.BeginOutputReadLine();
      }

      CollectedEvents? events = null;
      try
      {
        events = CollectEventsFromProcess(context, process.Id, launcherDto.BinaryStacksSavePath);
      }
      catch (Exception ex)
      {
        logger.LogError(ex, "Failed to collect events from {ProcessId}", process.Id);
      }
      finally
      {
        var timeoutMs = context.CommonContext.ProcessWaitTimeoutMs;
        if (!process.WaitForExit(timeoutMs))
        {
          logger.LogWarning("Failed to wait ({Timeout}ms) until process terminates naturally, killing it", timeoutMs);
          process.Kill();
          process.WaitForExit();
        }
      }

      if (context.CommonContext.PrintProcessOutput)
      {
        logger.LogInformation("Process output:");
        logger.LogInformation(sb.ToString());
      }

      if (!process.HasExited)
      {
        logger.LogError("The process {Id} somehow didn't exit", process.Id);
      }
      else
      {
        const string Message = "The process {Id} ({Path}) which was created by Procfiler exited";
        logger.LogInformation(Message, process.Id, commandName);
      }

      if (events.HasValue)
      {
        commandAction(events.Value);
      }
    }
    finally
    {
      if (context.CommonContext.ClearArtifacts &&
          launcherDto.BinaryStacksSavePath is { } binaryStacksSavePath)
      {
        PathUtils.ClearPathIfExists(binaryStacksSavePath, logger);
      }
    }
  }
}