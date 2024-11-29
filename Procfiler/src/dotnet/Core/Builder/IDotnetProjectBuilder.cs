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
  ProjectBuildOutputPath TempPath,
  bool SelfContained,
  string? AdditionalBuildArgs
);

public abstract record ProjectBuildOutputPath
{
  public static DefaultNetFolder DefaultNetFolder { get; } = new();
  public static RandomTempPath RandomTempPath { get; } = new();

  public static SpecifiedTempPath SpecifiedTempPath(string tempPath) => new(tempPath);
}

public sealed record DefaultNetFolder : ProjectBuildOutputPath;

public sealed record RandomTempPath : ProjectBuildOutputPath;

public sealed record SpecifiedTempPath(string TempFolderPath) : ProjectBuildOutputPath;

public interface IDotnetProjectBuilder
{
  BuildResult? TryBuildDotnetProject(ProjectBuildInfo projectBuildInfo);
}