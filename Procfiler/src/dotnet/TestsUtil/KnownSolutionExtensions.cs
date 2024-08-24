using Core.Builder;
using Core.InstrumentalProfiler;
using Core.Utils;

namespace TestsUtil;

public static class KnownSolutionExtensions
{
  public static ProjectBuildInfo CreateProjectBuildInfo(this KnownSolution knownSolution)
  {
    var solutionsDir = TestPaths.CreatePathToSolutionsSource();
    var csprojPath = Path.Combine(solutionsDir, knownSolution.Name, knownSolution.Name + ".csproj");
    return new ProjectBuildInfo(
      csprojPath, knownSolution.Tfm, BuildConfiguration.Debug, InstrumentationKind.None,
      true, PathUtils.CreateTempFolderPath(), false, null);
  }
}