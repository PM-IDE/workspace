using System.CommandLine;
using System.CommandLine.Invocation;
using Core.Builder;
using Core.CommandLine;
using Core.Utils;

namespace ProcfilerOnline.Commands;

public record CollectEventsOnlineContext(
  string DllFilePath,
  string OutputBxesFilePath
);

public class CollectEventsOnlineCommand(IProcfilerLogger logger) : ICommandWithContext<CollectEventsOnlineContext>
{
  private static Option<string> DllPathOption { get; } = new("--dll-path", "The path to dll to profile");
  private static Option<string> OutputPath { get; } = new("--output-path", "The output path for bXES file");


  public void Execute(CollectEventsOnlineContext context)
  {
  }

  public int Invoke(InvocationContext context) =>
    CommandLineUtils.TransformAndExecute(context, logger, Execute, parseResult =>
    {
      parseResult.AssertAllOptionsArePresent([DllPathOption, OutputPath]);

      return new CollectEventsOnlineContext(
        parseResult.GetValueForOption(DllPathOption)!,
        parseResult.GetValueForOption(OutputPath)!
      );
    });

  public Task<int> InvokeAsync(InvocationContext context) => Task.Run(() => Invoke(context));

  public Command CreateCommand()
  {
    var command = new Command("collect-online", "Collect events online from launched .NET dll");
    command.AddOption(DllPathOption);
    command.AddOption(OutputPath);

    return command;
  }
}