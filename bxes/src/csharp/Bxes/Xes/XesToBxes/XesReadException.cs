using System.Xml;

namespace Bxes.Xes.XesToBxes;

public class XesReadException(XmlReader reader, string message) : BxesException
{
  public override string Message { get; } = XesReadExceptionUtil.CreateExceptionMessage(message, reader);
}

public static class XesReadExceptionUtil
{
  public static string CreateExceptionMessage(string message, XmlReader reader) =>
    $"{message}, content: {reader.ReadOuterXml()}";
}