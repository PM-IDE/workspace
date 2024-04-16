using System.Xml;
using Bxes.Logging;
using Bxes.Xes.XesToBxes;

namespace Bxes.Utils;

public static class LoggerExtensions
{
  public static void LogWarning(this ILogger logger, XmlReader reader, string message)
  {
    logger.LogWarning(XesReadExceptionUtil.CreateExceptionMessage(message, reader));
  }
}