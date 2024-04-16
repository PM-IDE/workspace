using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.Documentation;

public interface IDocumentationCreator
{
  void CreateDocumentation(string documentationFolder);
}

[AppComponent]
public class DocumentationCreatorImpl(
  IEnumerable<IMarkdownDocumentationProvider> providers, IProcfilerLogger logger) : IDocumentationCreator
{
  public void CreateDocumentation(string documentationFolder)
  {
    foreach (var mdDocument in providers.Select(provider => provider.CreateDocumentationFile()))
    {
      var path = Path.Combine(documentationFolder, mdDocument.Name);
      if (!path.EndsWith(".md"))
      {
        path += ".md";
      }

      using var fs = File.OpenWrite(path);
      fs.SetLength(0);
      using var sw = new StreamWriter(fs);
      sw.Write(mdDocument.Serialize(new StringBuilder()));
    }
  }
}