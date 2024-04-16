namespace Procfiler.Commands.CollectClrEvents.Context;

public interface IParseResultInfoProvider
{
  T? TryGetOptionValue<T>(Option<T> option);
}

public class ParseResultInfoProviderImpl(ParseResult parseResult) : IParseResultInfoProvider
{
  public T? TryGetOptionValue<T>(Option<T> option)
  {
    return parseResult.GetValueForOption(option);
  }
}