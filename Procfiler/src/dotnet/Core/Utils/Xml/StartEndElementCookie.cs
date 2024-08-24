using System.Xml;

namespace Core.Utils.Xml;

public readonly struct StartEndElementCookie : IDisposable
{
  public static StartEndElementCookie CreateStartEndElement(
    XmlWriter xmlWriter, string? prefix, string tagName, string? @namespace)
  {
    xmlWriter.WriteStartElement(prefix, tagName, @namespace);
    return new StartEndElementCookie(xmlWriter);
  }


  private readonly XmlWriter myXmlWriter;


  private StartEndElementCookie(XmlWriter xmlWriter)
  {
    myXmlWriter = xmlWriter;
  }


  public void Dispose() => myXmlWriter.WriteEndElement();
}