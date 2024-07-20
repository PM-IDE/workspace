using Core.Container;
using Core.Utils;
using Procfiler.Core;
using Procfiler.Core.Documentation;
using Procfiler.Core.Exceptions;
using Procfiler.Utils;

namespace Procfiler.Commands.Documentation;

[CommandLineCommand]
public class CreateDocumentationCommand(
  IDocumentationCreator documentationCreator,
  IProcfilerLogger logger) : IVisibleToUserCommand
{
  private readonly Option<string> myOutputPathOption =
    new("-o", "The path to the directory into which the documentation will be generated")
    {
      IsRequired = true
    };


  public int Invoke(InvocationContext context)
  {
    try
    {
      var outputPath = context.ParseResult.GetValueForOption(myOutputPathOption) ??
                       throw new MissingOptionException(myOutputPathOption);

      documentationCreator.CreateDocumentation(outputPath);
      return 0;
    }
    catch (Exception ex)
    {
      logger.LogError(ex, "Error occured while creating documentation");
      return 1;
    }
  }

  public Task<int> InvokeAsync(InvocationContext context) => Task.Run(() => Invoke(context));

  public Command CreateCommand()
  {
    var command = new Command(
      "create-documentation", "Generate documentation for filters, mutators and other application components");

    command.AddOption(myOutputPathOption);
    command.Handler = this;
    return command;
  }
}