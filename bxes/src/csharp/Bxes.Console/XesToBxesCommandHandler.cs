using System.CommandLine.Parsing;
using Bxes.Logging;
using Bxes.Xes;
using Bxes.Xes.XesToBxes;

namespace Bxes.Console;

internal class XesToBxesCommandHandler(ILogger logger) : ConvertCommandHandlerBase
{
  protected override IBetweenFormatsConverter CreateConverter(ParseResult result) =>
    new XesToBxesConverter(
      logger,
      result.GetValueForOption(Options.BestBxesCompression),
      result.GetValueForOption(Options.WriteXesToBxesStatistics)
    );
}