using System.CommandLine;
using System.CommandLine.Invocation;
using System.Text.RegularExpressions;
using Core.Collector;
using Core.CommandLine;
using Core.Container;
using Core.Utils;
using ProcfilerOnline.Core;

namespace ProcfilerOnline.Commands;

public abstract record CollectEventsOnlineBaseContext(
  Regex? TargetMethodsRegex,
  Regex? MethodsFilterRegex,
  ProvidersCategoryKind Providers,
  ulong EventsFlushThreshold,
  bool RemoveFirstMoveNextFrames
)
{
  public abstract string ApplicationName { get; }
}

public sealed record CollectEventsOnlineFromDllContext(
  string DllFilePath,
  Regex? TargetMethodsRegex,
  Regex? MethodsFilterRegex,
  ProvidersCategoryKind Providers,
  ulong EventsFlushThreshold,
  bool RemoveFirstMoveNextFrames
) : CollectEventsOnlineBaseContext(TargetMethodsRegex, MethodsFilterRegex, Providers, EventsFlushThreshold, RemoveFirstMoveNextFrames)
{
  public override string ApplicationName { get; } = Path.GetFileNameWithoutExtension(DllFilePath);
}

public sealed record CollectEventsOnlineFromCsprojContext(
  string CsprojPath,
  Regex? TargetMethodsRegex,
  Regex? MethodsFilterRegex,
  ProvidersCategoryKind Providers,
  ulong EventsFlushThreshold,
  bool RemoveFirstMoveNextFrames
) : CollectEventsOnlineBaseContext(TargetMethodsRegex, MethodsFilterRegex, Providers, EventsFlushThreshold, RemoveFirstMoveNextFrames)
{
  public override string ApplicationName { get; } = Path.GetFileNameWithoutExtension(CsprojPath);
}

[AppComponent]
public class CollectEventsOnlineCommand(
  IProcfilerLogger logger,
  IClrOnlineEventsProcessor processor) : ICommandWithContext<CollectEventsOnlineBaseContext>
{
  private static Option<string> DllPathOption { get; } = new("-dll-path", "The path to dll to profile");
  private static Option<string> CsprojOption { get; } = new("-csproj", "The project to execute");

  private static Option<string> TargetMethodsRegex { get; } =
    new("--target-methods-regex", "The regular expression which specified target methods");

  private static Option<string> MethodsFilterRegex { get; } = new("--methods-filter-regex", "The regular expression to filter methods");

  private static Option<ProvidersCategoryKind> ProvidersOption { get; } =
    new("--providers", static () => ProvidersCategoryKind.All, "Providers which will be used for collecting events");

  private static Option<ulong> EventsFlushThreshold { get; } =
    new("--flush-threshold", static () => 10_000, "After this number of events stored in the trace it will be flushed");

  private static Option<bool> RemoveFirstMoveNextFrames { get; } =
    new("--remove-first-move-next-frames", static () => true, "Remove first MoveNext frames from async methods traces");


  public void Execute(CollectEventsOnlineBaseContext context)
  {
    processor.StartProfiling(context);
  }

  public int Invoke(InvocationContext context) =>
    CommandLineUtils.TransformAndExecute(context, logger, Execute, parseResult =>
    {
      var dllPath = parseResult.GetValueForOption(DllPathOption);
      var csprojPath = parseResult.GetValueForOption(CsprojOption);

      return (CollectEventsOnlineBaseContext)((dllPath, command: csprojPath) switch
      {
        ({ }, null) => new CollectEventsOnlineFromDllContext(
          dllPath,
          CreateRegex(parseResult.GetValueForOption(TargetMethodsRegex)),
          CreateRegex(parseResult.GetValueForOption(MethodsFilterRegex)),
          parseResult.GetValueForOption(ProvidersOption),
          parseResult.GetValueForOption(EventsFlushThreshold),
          parseResult.GetValueForOption(RemoveFirstMoveNextFrames)
        ),
        (null, { }) => new CollectEventsOnlineFromCsprojContext(
          csprojPath,
          CreateRegex(parseResult.GetValueForOption(TargetMethodsRegex)),
          CreateRegex(parseResult.GetValueForOption(MethodsFilterRegex)),
          parseResult.GetValueForOption(ProvidersOption),
          parseResult.GetValueForOption(EventsFlushThreshold),
          parseResult.GetValueForOption(RemoveFirstMoveNextFrames)
        ),
        _ => throw new OneOfFollowingOptionsMustBeSpecifiedException([DllPathOption, CsprojOption])
      });
    });

  private static Regex? CreateRegex(string? stringRegex) => stringRegex is { } ? new Regex(stringRegex) : null;

  public Task<int> InvokeAsync(InvocationContext context) => Task.Run(() => Invoke(context));

  public Command CreateCommand()
  {
    var command = new Command("collect-online", "Collect events online from launched .NET dll");
    command.AddOption(DllPathOption);
    command.AddOption(CsprojOption);
    command.AddOption(TargetMethodsRegex);
    command.AddOption(MethodsFilterRegex);
    command.AddOption(ProvidersOption);
    command.AddOption(EventsFlushThreshold);

    command.Handler = this;

    return command;
  }
}