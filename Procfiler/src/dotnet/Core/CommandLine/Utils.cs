using System.CommandLine;
using System.CommandLine.Invocation;
using System.CommandLine.Parsing;
using Core.Utils;
using Microsoft.Extensions.Logging;

namespace Core.CommandLine;


public static class ParseResultExtensions
{
  public static bool HasErrors(this ParseResult parseResult, IProcfilerLogger logger)
  {
    var errors = parseResult.Errors;
    if (errors.Count <= 0) return false;

    foreach (var error in errors)
    {
      logger.LogError(error.Message);
    }

    return true;
  }

  public static void AssertAllOptionsArePresent(this ParseResult parseResult, IEnumerable<Option> options)
  {
    foreach (var option in options)
    {
      if (!parseResult.HasOption(option))
      {
        throw new MissingOptionException(option);
      }
    }
  }
}

public static class CommandLineUtils
{
  public static int TransformAndExecute<T>(
    InvocationContext context, IProcfilerLogger logger, Action<T> action, Func<ParseResult, T> transform)
  {
    var parseResult = context.ParseResult;
    if (parseResult.HasErrors(logger)) return -1;

    var newContext = transform(parseResult);

    try
    {
      action(newContext);
    }
    catch (Exception ex)
    {
      logger.LogError(ex.Message);
      return -1;
    }

    return 0;
  }
}