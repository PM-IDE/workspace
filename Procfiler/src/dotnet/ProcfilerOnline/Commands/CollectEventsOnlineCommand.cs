using System.CommandLine;
using System.CommandLine.Invocation;
using System.Text.RegularExpressions;
using Core.CommandLine;
using Core.Container;
using Core.Utils;
using ProcfilerOnline.Core;

namespace ProcfilerOnline.Commands;

public record CollectEventsOnlineContext(
  string DllFilePath,
  string OutputBxesFilePath,
  Regex? TargetMethodsRegex,
  Regex? MethodsFilterRegex
);

[AppComponent]
public class CollectEventsOnlineCommand(IProcfilerLogger logger, IOnlineEventsProcessor processor) : ICommandWithContext<CollectEventsOnlineContext>
{
  private static Option<string> DllPathOption { get; } = new("--dll-path", "The path to dll to profile");
  private static Option<string> OutputPath { get; } = new("--output-path", "The output path for bXES file");
  private static Option<string> TargetMethodsRegex { get; } = new("--target-methods-regex", "The regular expression which specified target methods");
  private static Option<string> MethodsFilterRegex { get; } = new("--methods-filter-regex", "THe regular expression to filter methods");


  public void Execute(CollectEventsOnlineContext context)
  {
    processor.StartProfiling(context);
  }

  public int Invoke(InvocationContext context) =>
    CommandLineUtils.TransformAndExecute(context, logger, Execute, parseResult =>
    {
      parseResult.AssertAllOptionsArePresent([DllPathOption, OutputPath]);

      return new CollectEventsOnlineContext(
        parseResult.GetValueForOption(DllPathOption)!,
        parseResult.GetValueForOption(OutputPath)!,
        CreateRegex(parseResult.GetValueForOption(TargetMethodsRegex)),
        CreateRegex(parseResult.GetValueForOption(MethodsFilterRegex))
      );
    });

  private static Regex? CreateRegex(string? stringRegex) => stringRegex is { } ? new Regex(stringRegex) : null;

  public Task<int> InvokeAsync(InvocationContext context) => Task.Run(() => Invoke(context));

  public Command CreateCommand()
  {
    var command = new Command("collect-online", "Collect events online from launched .NET dll");
    command.AddOption(DllPathOption);
    command.AddOption(OutputPath);
    command.AddOption(TargetMethodsRegex);
    command.AddOption(MethodsFilterRegex);

    command.Handler = this;

    return command;
  }
}