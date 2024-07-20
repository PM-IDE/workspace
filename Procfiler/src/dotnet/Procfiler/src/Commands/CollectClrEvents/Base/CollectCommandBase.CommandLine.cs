using System.CommandLine.Binding;
using Core.Builder;
using Core.CommandLine;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Utils;

namespace Procfiler.Commands.CollectClrEvents.Base;

public abstract partial class CollectCommandBase
{
  public Task<int> InvokeAsync(InvocationContext context) => Task.Run(() => Invoke(context));

  public int Invoke(InvocationContext context) =>
    CommandLineUtils.TransformAndExecute(context, Logger, Execute, parseResult =>
    {
      CheckForPidOrExePathOrThrow(parseResult);
      return CreateCollectClrContextFrom(parseResult);
    });

  public Command CreateCommand()
  {
    var command = CreateCommandInternal();
    command.Handler = this;
    AddCommonOptions(command);
    return command;
  }

  protected abstract Command CreateCommandInternal();

  private CollectingClrEventsCommonContext CreateCommonContext(ParseResult parseResult)
  {
    if (parseResult.GetValueForOption(OutputPathOption) is not { } outputPath)
    {
      throw new MissingOptionException(OutputPathOption);
    }

    var fileFormat = parseResult.GetValueForOption(OutputFileFormatOption);
    var duration = parseResult.GetValueForOption(DurationOption);
    var timeout = parseResult.GetValueForOption(TimeoutOption);
    var clearBefore = parseResult.GetValueForOption(ClearPathBeforeOption);
    var category = parseResult.GetValueForOption(ProvidersCategoryOption);
    var arguments = parseResult.GetValueForOption(ArgumentsOption) ?? string.Empty;
    var printOutput = parseResult.GetValueForOption(PrintProcessOutputOption);
    var methodsFilterRegex = parseResult.GetValueForOption(FilterOption);
    var processWaitTimeoutMs = parseResult.GetValueForOption(ProcessWaitTimeoutOption);
    var useCppProfiler = parseResult.GetValueForOption(UseCppProfilerOption);
    var useDuringRuntimeFiltering = parseResult.GetValueForOption(UseDuringRuntimeMethodsFiltering);
    var cppProfilerUseConsoleLogging = parseResult.GetValueForOption(UseCppProfilerConsoleLogging);
    var clearArtifacts = parseResult.GetValueForOption(ClearArtifactsOption);
    var writeAllEventMetadata = parseResult.GetValueForOption(WriteAllEventMetadata);
    var logSerializationFormat = parseResult.GetValueForOption(LogSerializationFormatOption);

    var serializationCtx = new SerializationContext(fileFormat);
    var parseResultInfoProvider = new ParseResultInfoProviderImpl(parseResult);

    return new CollectingClrEventsCommonContext(
      outputPath, serializationCtx, parseResultInfoProvider, arguments, category, clearBefore, duration, timeout,
      printOutput, methodsFilterRegex, processWaitTimeoutMs, useCppProfiler, useDuringRuntimeFiltering, cppProfilerUseConsoleLogging,
      clearArtifacts, writeAllEventMetadata, logSerializationFormat);
  }

  private CollectClrEventsContext CreateCollectClrContextFrom(ParseResult parseResult)
  {
    var commonContext = CreateCommonContext(parseResult);

    if (parseResult.HasOption(CommandNameOption))
    {
      if (parseResult.GetValueForOption(CommandNameOption) is not { } command)
      {
        Logger.LogError("{Option} was null", CommandNameOption.Name);
        throw new ArgumentNullException(nameof(CommandNameOption));
      }

      return new CollectClrEventsFromCommandContext(command, TryGetArgumentsOrThrow(parseResult), commonContext);
    }

    if (parseResult.HasOption(ProcessIdOption))
    {
      if (parseResult.HasOption(RepeatOption))
      {
        Logger.LogWarning(
          "The repeat option was specified when attaching to already running process, for now this option will be ignored");
      }

      var pid = parseResult.GetValueForOption(ProcessIdOption);
      return new CollectClrEventsFromRunningProcessContext(pid, commonContext);
    }

    if (parseResult.HasOption(PathToCsprojOption))
    {
      var pathToCsproj = parseResult.GetValueForOption(PathToCsprojOption);
      if (!File.Exists(pathToCsproj))
      {
        throw new ArgumentOutOfRangeException(nameof(pathToCsproj));
      }

      var projectBuildInfo = CreateProjectBuildInfo(parseResult, pathToCsproj);
      if (parseResult.HasOption(RepeatOption) &&
          parseResult.GetValueForOption(RepeatOption) > 1 &&
          parseResult.HasOption(ArgumentsFileOption) &&
          !Equals(parseResult.GetValueForOption(ArgumentsFileOption), ArgumentsFileOption.GetDefaultValue()))
      {
        Logger.LogError("Executing with arguments file and repeat count is not yet supported");
        throw new ArgumentOutOfRangeException();
      }

      if (TryGetArgumentsOrThrow(parseResult) is { } arguments)
      {
        return new CollectClrEventsFromExeWithArguments(projectBuildInfo, commonContext, arguments);
      }

      if (parseResult.HasOption(RepeatOption))
      {
        var repeatCount = parseResult.GetValueForOption(RepeatOption);
        if (repeatCount < 1)
        {
          throw new ArgumentOutOfRangeException(nameof(RepeatOption), "The --repeat must be greater or equal than 1");
        }

        return new CollectClrEventsFromExeWithRepeatContext(projectBuildInfo, repeatCount, commonContext);
      }

      return new CollectClrEventsFromExeContext(projectBuildInfo, commonContext);
    }

    throw new ArgumentOutOfRangeException();
  }

  private IReadOnlyList<string>? TryGetArgumentsOrThrow(ParseResult parseResult)
  {
    if (parseResult.HasOption(ArgumentsFileOption))
    {
      var filePath = parseResult.GetValueForOption(ArgumentsFileOption);
      if (!Equals(filePath, ArgumentsFileOption.GetDefaultValue()))
      {
        if (!File.Exists(filePath))
        {
          Logger.LogError("Invalid path to arguments file: {Path}", filePath);
          throw new FileNotFoundException("Failed to find argument file", filePath);
        }

        return File.ReadAllLines(filePath);
      }
    }

    return null;
  }

  private ProjectBuildInfo CreateProjectBuildInfo(ParseResult parseResult, string pathToCsproj)
  {
    var tfm = parseResult.GetValueForOption(TfmOption);
    Debug.Assert(tfm is { });

    var buildConfiguration = parseResult.GetValueForOption(ConfigurationOption);
    var instrumentationKind = parseResult.GetValueForOption(InstrumentCodeOption);
    var selfContained = parseResult.GetValueForOption(SelfContainedOption);

    var additionalBuildArgs = parseResult.GetValueForOption(AdditionalBuildArgsOption);
    if (Equals(additionalBuildArgs, ((IValueDescriptor)TempPathOption).GetDefaultValue()))
    {
      additionalBuildArgs = null;
    }

    var tempPath = parseResult.GetValueForOption(TempPathOption);
    if (Equals(tempPath, ((IValueDescriptor)TempPathOption).GetDefaultValue()))
    {
      tempPath = null;
    }

    var clearActivities = parseResult.GetValueForOption(ClearArtifactsOption);
    return new ProjectBuildInfo(
      pathToCsproj, tfm, buildConfiguration, instrumentationKind, clearActivities, tempPath, selfContained, additionalBuildArgs);
  }

  private void CheckForPidOrExePathOrThrow(ParseResult parseResult)
  {
    if (!parseResult.HasOption(ProcessIdOption) &&
        !parseResult.HasOption(PathToCsprojOption) &&
        !parseResult.HasOption(CommandNameOption))
    {
      throw new OneOfFollowingOptionsMustBeSpecifiedException(ProcessIdOption, PathToCsprojOption);
    }
  }
}