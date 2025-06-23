using System.Reflection;
using Aspire.Hosting;
using Aspire.Hosting.ApplicationModel;

namespace ProcfilerOnline.Aspire;

public static class DistributedApplicationBuilderExtensions
{
  public static IResourceBuilder<ExecutableResource> AddLocalProcfilerExecutable<TProject>(
    this IDistributedApplicationBuilder builder,
    string name,
    string localProcfilerExecutablePath
  ) where TProject : IProjectMetadata, new()
  {
    var projectPath = new TProject().ProjectPath;
    var projectDir = Path.GetDirectoryName(projectPath)!;

    return builder.AddExecutable(
      $"procfiler-{name}",
      localProcfilerExecutablePath,
      projectDir,
      "collect-online",
      "-command",
      $"""
      "dotnet run {projectDir}"
      """
    );
  }
}