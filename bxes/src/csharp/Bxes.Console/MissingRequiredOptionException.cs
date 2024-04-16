namespace Bxes.Console;

internal class MissingRequiredOptionException(string optionName) : Exception
{
  public override string Message { get; } = $"The required option {optionName} was not supplied";
}