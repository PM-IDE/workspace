using Bxes.Logging;

namespace Bxes.Utils;

public static class BxesDefaultLoggerFactory
{
  public static ILogger Create() => new BxesLogger();
}