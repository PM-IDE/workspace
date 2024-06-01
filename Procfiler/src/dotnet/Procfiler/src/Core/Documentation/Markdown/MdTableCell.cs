namespace Procfiler.Core.Documentation.Markdown;

public record MdTableCell(string Content) : IMdDocumentPart
{
  public int ContentLength => Content.Length;


  public StringBuilder Serialize(StringBuilder sb) => sb.Append(Content);
}