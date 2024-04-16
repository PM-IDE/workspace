namespace Procfiler.Core.InstrumentalProfiler.DepsJson;

public class DepsJsonFile
{
  public TopLevelRuntimeTarget RuntimeTarget { get; set; }
  public CompilationOptions CompilationOptions { get; set; }
  public Targets Targets { get; set; }
  public Libraries Libraries { get; set; }
  public Runtimes Runtimes { get; set; }
}

public class CompilationOptions
{
  public required List<string> Options { get; init; }
}

public class TopLevelRuntimeTarget
{
  public NameWithVersion NameWithVersion { get; set; }
  public string Signature { get; set; }
}

public class Targets
{
  public required List<TargetItem> TargetsList { get; init; }
}

public class Libraries
{
  public required List<LibraryEntry> LibrariesList { get; init; }
}

public class LibraryEntry
{
  public string Name { get; set; }

  public string? Type { get; set; }
  public string? Path { get; set; }
  public bool? Serviceable { get; set; }
  public string? Sha512 { get; set; }
  public string? HashPath { get; set; }
}

public class Runtimes
{
  public required List<Runtime> RuntimesList { get; init; }
}

public class Runtime
{
  public string Rid { get; set; }
  public required List<string> Fallbacks { get; init; }
}

public class TargetItem
{
  public NameWithVersion NameWithVersion { get; set; }
  public List<TargetDependency> Dependencies { get; init; }
}

public class TargetDependency
{
  public string Name { get; set; }
  public required List<DependencyOfTargetDependency> Dependencies { get; init; }
  public required List<RuntimeTarget> RuntimeTargets { get; init; }
  public required List<FileInfo> Runtime { get; init; }
  public required List<FileInfo> Native { get; init; }
}

public class NameWithVersion
{
  //todo: parse version
  public static NameWithVersion Parse(string rawName) => new()
  {
    Name = rawName,
    Version = null
  };

  public static NameWithVersion Create(string name, string version) => new()
  {
    Name = name,
    Version = version
  };


  public required string Name { get; init; }
  public required string? Version { get; init; }


  public override string ToString()
  {
    return Name;
  }
}

public class DependencyOfTargetDependency
{
  public NameWithVersion NameWithVersion { get; init; }
}

public class FileInfo
{
  public string Name { get; set; }
  public Version? AssemblyVersion { get; set; }
  public Version? FileVersion { get; set; }
}

public class RuntimeTarget
{
  public string Name { get; set; }
  public string Rid { get; set; }
  public string AssetType { get; set; }
  public Version AssemblyVersion { get; set; }
}