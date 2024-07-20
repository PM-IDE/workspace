using Core.Utils;
using Mono.Cecil;
using Procfiler.Core.Exceptions;
using Procfiler.Utils;

namespace Procfiler.Core.InstrumentalProfiler;

public record AssemblyDefWithPath(AssemblyDefinition Assembly, string PhysicalPath);

public class SelfContainedTypeCache(IProcfilerLogger logger, string contextFolder)
{
  private readonly Dictionary<string, TypeDefinition> myCache = new();
  private readonly Dictionary<string, AssemblyDefWithPath> myAssemblies = new();
  private readonly FolderBasedAssemblyResolver myResolver = new(logger, contextFolder);


  public IReadOnlyDictionary<string, AssemblyDefWithPath> Assemblies => myAssemblies;
  public IReadOnlyDictionary<string, TypeDefinition> Types => myCache;


  public void Initialize()
  {
    myResolver.Initialize();
    foreach (var (fullName, assemblyDefWithPath) in myResolver.Assemblies)
    {
      myAssemblies[fullName] = assemblyDefWithPath;

      foreach (var module in assemblyDefWithPath.Assembly.Modules)
      {
        foreach (var type in module.Types)
        {
          if (myCache.TryGetValue(type.FullName, out var existingType))
          {
            logger.LogWarning("Already have type def for {FullName}: {TypeDef}", type.FullName, existingType);
            continue;
          }

          myCache[type.FullName] = type;
        }
      }
    }
  }
}

public class FailedToResolveAssemblyException(IAssemblyResolver resolver, AssemblyNameReference reference)
  : ProcfilerException($"Failed to resolve {reference.FullName} from resolver: {resolver.GetType().Name}");