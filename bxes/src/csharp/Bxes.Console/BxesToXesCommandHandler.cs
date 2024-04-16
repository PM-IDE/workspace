using System.CommandLine.Parsing;
using Bxes.Xes;
using Bxes.Xes.BxesToXes;

namespace Bxes.Console;

internal class BxesToXesCommandHandler : ConvertCommandHandlerBase
{
  protected override IBetweenFormatsConverter CreateConverter(ParseResult result) => new BxesToXesConverter();
}