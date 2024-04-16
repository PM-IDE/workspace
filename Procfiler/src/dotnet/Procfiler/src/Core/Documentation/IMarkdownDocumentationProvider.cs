using Procfiler.Core.Documentation.Markdown;

namespace Procfiler.Core.Documentation;

public interface IMarkdownDocumentationProvider
{
  MdDocument CreateDocumentationFile();
}

public interface IMdDocumentPart
{
  StringBuilder Serialize(StringBuilder sb);
}