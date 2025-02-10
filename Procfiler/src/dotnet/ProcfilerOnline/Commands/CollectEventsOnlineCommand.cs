using System.CommandLine;
using System.CommandLine.Invocation;
using System.Text.RegularExpressions;
using Core.Collector;
using Core.CommandLine;
using Core.Container;
using Core.Utils;
using ProcfilerOnline.Core;

namespace ProcfilerOnline.Commands;

public record CollectEventsOnlineContext(
  string DllFilePath,
  Regex? TargetMethodsRegex,
  Regex? MethodsFilterRegex,
  ProvidersCategoryKind Providers,
  ulong EventsFlushThreshold
)
{
  public string ApplicationName { get; } = Path.GetFileNameWithoutExtension(DllFilePath);
}

[AppComponent]
public class CollectEventsOnlineCommand(
  IProcfilerLogger logger,
  IClrOnlineEventsProcessor processor) : ICommandWithContext<CollectEventsOnlineContext>
{
  private static Option<string> DllPathOption { get; } = new("-dll-path", "The path to dll to profile");

  private static Option<string> TargetMethodsRegex { get; } =
    new("--target-methods-regex", "The regular expression which specified target methods");

  private static Option<string> MethodsFilterRegex { get; } = new("--methods-filter-regex", "The regular expression to filter methods");

  private static Option<ProvidersCategoryKind> ProvidersOption { get; } =
    new("--providers", static () => ProvidersCategoryKind.All, "Providers which will be used for collecting events");

  private static Option<ulong> EventsFlushThreshold { get; } =
    new("--flush-threshold", static () => 10_000, "After this number of events stored in the trace it will be flushed");


  public void Execute(CollectEventsOnlineContext context)
  {
    processor.StartProfiling(context);
  }

  public int Invoke(InvocationContext context) =>
    CommandLineUtils.TransformAndExecute(context, logger, Execute, parseResult =>
    {
      parseResult.AssertAllOptionsArePresent([DllPathOption]);

      return new CollectEventsOnlineContext(
        parseResult.GetValueForOption(DllPathOption)!,
        CreateRegex(parseResult.GetValueForOption(TargetMethodsRegex)),
        CreateRegex(parseResult.GetValueForOption(MethodsFilterRegex)),
        parseResult.GetValueForOption(ProvidersOption),
        parseResult.GetValueForOption(EventsFlushThreshold)
      );
    });

  private static Regex? CreateRegex(string? stringRegex) => stringRegex is { } ? new Regex(stringRegex) : null;

  public Task<int> InvokeAsync(InvocationContext context) => Task.Run(() => Invoke(context));

  public Command CreateCommand()
  {
    var command = new Command("collect-online", "Collect events online from launched .NET dll");
    command.AddOption(DllPathOption);
    command.AddOption(TargetMethodsRegex);
    command.AddOption(MethodsFilterRegex);
    command.AddOption(ProvidersOption);
    command.AddOption(EventsFlushThreshold);

    command.Handler = this;

    return command;
  }
}