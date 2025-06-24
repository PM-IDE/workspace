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

    var projectResource = builder.AddProject<TProject>(name);
    var executableResource = builder.AddExecutable(
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

    foreach (var resourceAnnotation in projectResource.Resource.Annotations)
    {
      executableResource.WithAnnotation(resourceAnnotation);
    }

    builder.Resources.Remove(projectResource.Resource);

    return executableResource;
  }
}