using Mono.Cecil;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.InstrumentalProfiler.DepsJson;

public interface IDepsJsonPatcher
{
  Task AddAssemblyReferenceAsync(
    AssemblyDefinition originalAssembly, string depsJsonPath, string assemblyToAddName, Version assemblyToAddVersion);
}

[AppComponent]
public class DepsJsonPatcherImpl(IDepsJsonReader depsJsonReader, IDepsJsonWriter depsJsonWriter) : IDepsJsonPatcher
{
  public async Task AddAssemblyReferenceAsync(
    AssemblyDefinition originalAssembly, string depsJsonPath, string assemblyToAddName, Version assemblyToAddVersion)
  {
    var depsJsonFile = await depsJsonReader.ReadOrThrowAsync(depsJsonPath);
    var originalAssemblyVersion = originalAssembly.Name.Version.ToString(3);
    var asmToAddNameWithVersion = $"{assemblyToAddName}/{assemblyToAddVersion.ToString(3)}";

    foreach (var item in depsJsonFile.Targets.TargetsList.Where(list => list.Dependencies.Count > 0))
    {
      foreach (var targetDependency in item.Dependencies)
      {
        if (targetDependency.Name == $"{originalAssembly.Name.Name}/{originalAssemblyVersion}")
        {
          targetDependency.Dependencies.Add(new DependencyOfTargetDependency
          {
            NameWithVersion = new NameWithVersion
            {
              Name = assemblyToAddName,
              Version = assemblyToAddVersion.ToString()
            }
          });
        }
      }

      item.Dependencies.Add(new TargetDependency
      {
        Name = asmToAddNameWithVersion,
        Dependencies = EmptyCollections<DependencyOfTargetDependency>.EmptyList,
        Native = EmptyCollections<FileInfo>.EmptyList,
        RuntimeTargets = EmptyCollections<RuntimeTarget>.EmptyList,
        Runtime = [new() { Name = $"{assemblyToAddName}.dll" }]
      });
    }

    depsJsonFile.Libraries.LibrariesList.Add(new LibraryEntry
    {
      Name = asmToAddNameWithVersion,
      Sha512 = string.Empty,
      Serviceable = false,
      Type = "project"
    });

    await depsJsonWriter.WriteAsync(depsJsonPath, depsJsonFile);
  }
}