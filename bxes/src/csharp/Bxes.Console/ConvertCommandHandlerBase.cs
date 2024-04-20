using System.CommandLine.Invocation;
using System.CommandLine.Parsing;
using Bxes.Xes;

namespace Bxes.Console;

internal abstract class ConvertCommandHandlerBase : ICommandHandler
{
  public int Invoke(InvocationContext context)
  {
    var filePath = context.ParseResult.GetValueOrThrow(Options.PathOption);
    var outputFilePath = context.ParseResult.GetValueOrThrow(Options.OutputPathOption);

    CreateConverter(context.ParseResult).Convert(filePath, outputFilePath);

    return 0;
  }

  public Task<int> InvokeAsync(InvocationContext context) => Task.Run(() => Invoke(context));

  protected abstract IBetweenFormatsConverter CreateConverter(ParseResult result);
}