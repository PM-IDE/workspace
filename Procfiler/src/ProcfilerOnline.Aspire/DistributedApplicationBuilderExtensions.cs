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
    var projectName = Path.GetFileName(projectPath);

    return builder.AddExecutable(
      $"procfiler-{name}",
      localProcfilerExecutablePath,
      Path.GetDirectoryName(projectPath)!,
      "collect-online",
      "-csproj",
      projectPath,
      "--target-methods-regex",
      projectName,
      "--methods-filter-regex",
      projectName
    );
  }
}