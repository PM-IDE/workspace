using Core.Builder;
using Core.CppProcfiler;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Core.CppProcfiler;

namespace Procfiler.Core.Processes;

public readonly struct DotnetProcessLauncherDto
{
  public required bool DefaultDiagnosticPortSuspend { get; init; }
  public required string PathToDotnetExecutable { get; init; }
  public required string Arguments { get; init; }
  public required bool RedirectOutput { get; init; }
  public required string? BinaryStacksSavePath { get; init; }
  public required string CppProcfilerPath { get; init; }
  public required string? MethodsFilterRegex { get; init; }
  public required CppProfilerMode CppProfilerMode { get; init; }
  public required bool UseDuringRuntimeFiltering { get; init; }
  public required bool CppProfilerUseConsoleLogging { get; init; }
  public required string WorkingDirectory { get; init; }
  public required bool ProduceObjectBinStacks { get; init; }


  public static DotnetProcessLauncherDto CreateFrom(
    CollectingClrEventsCommonContext context,
    BuildResult buildResult,
    ICppProcfilerLocator locator,
    IBinaryStackSavePathCreator savePathCreator)
  {
    var workingDirectory = Path.GetDirectoryName(buildResult.BuiltDllPath)!;
    var patchedContext = context with
    {
      Arguments = $"{buildResult.BuiltDllPath} {context.Arguments}"
    };

    var binStacksSavePath = context.CppProfilerMode.IsEnabled() switch
    {
      true => savePathCreator.CreateSavePath(buildResult, context.CppProfilerMode),
      false => null
    };

    return CreateInternal(patchedContext, locator, "dotnet", workingDirectory, binStacksSavePath);
  }

  private static DotnetProcessLauncherDto CreateInternal(
    CollectingClrEventsCommonContext context,
    ICppProcfilerLocator locator,
    string commandName,
    string workingDirectory,
    string? binStacksSavePath) => new()
  {
    DefaultDiagnosticPortSuspend = true,
    Arguments = context.Arguments,
    RedirectOutput = context.PrintProcessOutput,
    PathToDotnetExecutable = commandName,
    CppProcfilerPath = locator.FindCppProcfilerPath("CppProcfiler"),
    MethodsFilterRegex = context.CppProcfilerMethodsFilterRegex,
    CppProfilerMode = context.CppProfilerMode,
    UseDuringRuntimeFiltering = context.UseDuringRuntimeFiltering,
    CppProfilerUseConsoleLogging = context.CppProfilerUseConsoleLogging,
    WorkingDirectory = workingDirectory,
    BinaryStacksSavePath = binStacksSavePath,
    ProduceObjectBinStacks = context.ProduceObjectBinStacks
  };

  public static DotnetProcessLauncherDto CreateFrom(
    CollectingClrEventsCommonContext context,
    string commandName,
    ICppProcfilerLocator locator,
    IBinaryStackSavePathCreator savePathCreator)
  {
    var binStacksSavePath = context.CppProfilerMode.IsEnabled() switch
    {
      true => savePathCreator.CreateTempSavePath(context.CppProfilerMode),
      false => null
    };

    return CreateInternal(context, locator, commandName, string.Empty, binStacksSavePath);
  }
}