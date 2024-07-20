using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Utils;

namespace Procfiler.Core.Processes.Builder;

public readonly struct BuildResult(TempFolderCookie tempFolderCookie)
{
  public required string BuiltDllPath { get; init; }


  public void ClearUnderlyingFolder() => tempFolderCookie.Dispose();
}

public interface IDotnetProjectBuilder
{
  BuildResult? TryBuildDotnetProject(ProjectBuildInfo projectBuildInfo);
}