using Core.InstrumentalProfiler;
using Core.Utils;

namespace Core.Builder;

public readonly struct BuildResult(TempFolderCookie tempFolderCookie)
{
  public required string BuiltDllPath { get; init; }


  public void ClearUnderlyingFolder() => tempFolderCookie.Dispose();
}

public record struct ProjectBuildInfo(
  string CsprojPath,
  string Tfm,
  BuildConfiguration Configuration,
  InstrumentationKind InstrumentationKind,
  bool ClearArtifacts,
  string? TempPath,
  bool SelfContained,
  string? AdditionalBuildArgs
);

public interface IDotnetProjectBuilder
{
  BuildResult? TryBuildDotnetProject(ProjectBuildInfo projectBuildInfo);
}