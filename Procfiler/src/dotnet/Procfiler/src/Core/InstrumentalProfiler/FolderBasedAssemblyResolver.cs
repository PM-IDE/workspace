using Core.Utils;
using Mono.Cecil;
using Procfiler.Core.Constants;
using Procfiler.Utils;

namespace Procfiler.Core.InstrumentalProfiler;

public interface IAssembliesProvider
{
  IReadOnlyDictionary<string, AssemblyDefWithPath> Assemblies { get; }

  void Initialize();
}

public class FolderBasedAssemblyResolver(
  IProcfilerLogger logger,
  string contextFolder) : IAssemblyResolver, IAssembliesProvider
{
  private readonly Dictionary<string, AssemblyDefWithPath> myContextAssembliesToPaths = new();

  private bool myContextAssembliesInitialized;


  public IReadOnlyDictionary<string, AssemblyDefWithPath> Assemblies => myContextAssembliesToPaths;


  public AssemblyDefWithPath ResolveWithPath(AssemblyNameReference reference, ReaderParameters parameters)
  {
    InitializeContextAssembliesIfNeeded(parameters);
    if (myContextAssembliesToPaths.TryGetValue(reference.FullName, out var assemblyDefinition))
    {
      return assemblyDefinition;
    }

    throw new FailedToResolveAssemblyException(this, reference);
  }

  public AssemblyDefinition Resolve(AssemblyNameReference reference) => Resolve(reference, CreateReaderParameters());

  public AssemblyDefinition Resolve(AssemblyNameReference reference, ReaderParameters parameters) =>
    ResolveWithPath(reference, parameters).Assembly;

  private ReaderParameters CreateReaderParameters() => new()
  {
    ReadWrite = false,
    InMemory = true,
    ReadSymbols = false,
    AssemblyResolver = this
  };

  public void Initialize() => InitializeContextAssembliesIfNeeded(CreateReaderParameters());

  private void InitializeContextAssembliesIfNeeded(ReaderParameters readerParameters)
  {
    if (myContextAssembliesInitialized) return;

    foreach (var filePath in Directory.GetFiles(contextFolder))
    {
      if (!filePath.EndsWith(DotNetConstants.DllExtension)) continue;

      try
      {
        var assembly = AssemblyDefinition.ReadAssembly(filePath, readerParameters);
        if (assembly.FullName is null)
        {
          logger.LogWarning("Full name for assembly {Path} was null, skipping it", filePath);
          continue;
        }

        myContextAssembliesToPaths[assembly.FullName] = new AssemblyDefWithPath(assembly, filePath);
      }
      catch (Exception)
      {
        logger.LogWarning("Failed to read assembly {Path}", filePath);
      }
    }

    myContextAssembliesInitialized = true;
  }

  public void Dispose()
  {
  }
}