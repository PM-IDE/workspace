using Aspire.Hosting;
using Aspire.Hosting.ApplicationModel;

namespace ProcfilerOnline.Aspire;

public static class DistributedApplicationBuilderExtensions
{
  public class ProcfilerExecutableResource(string name, string command, string workingDirectory)
    : ExecutableResource(name, command, workingDirectory), IResourceWithServiceDiscovery;

  public static IResourceBuilder<ProcfilerExecutableResource> AddLocalProcfilerExecutable<TProject>(
    this IDistributedApplicationBuilder builder,
    string name,
    string localProcfilerExecutablePath,
    string? targetMethodsRegex = null,
    string? methodsFilterRegex = null
  ) where TProject : IProjectMetadata, new()
  {
    var projectPath = new TProject().ProjectPath;
    var projectName = Path.GetFileNameWithoutExtension(projectPath);

    var projectResource = builder
      .AddProject<TProject>(name)
      .WithEnvironment("ProduceEventsToKafka", "true")
      .WithEnvironment("ProduceBxesKafkaEvents", "true")
      .WithEnvironment("ProduceGcEvents", "false")
      .WithEnvironment("OnlineProcfilerSettings__KafkaSettings__TopicName", "my-topic")
      .WithEnvironment("OnlineProcfilerSettings__KafkaSettings__BootstrapServers", "localhost:9092");

    var procfilerExecutableResource = new ProcfilerExecutableResource(
      $"procfiler-{name}", localProcfilerExecutablePath, Path.GetDirectoryName(projectPath)!);

    var resourceBuilder = builder
      .AddResource(procfilerExecutableResource)
      .WithArgs(context => context.Args.AddRange([
        "collect-online",
        "-csproj",
        projectPath,
        "--target-methods-regex",
        targetMethodsRegex ?? projectName,
        "--methods-filter-regex",
        methodsFilterRegex ?? projectName
      ]));

    foreach (var resourceAnnotation in projectResource.Resource.Annotations)
    {
      resourceBuilder.WithAnnotation(resourceAnnotation);
    }

    builder.Resources.Remove(projectResource.Resource);

    return resourceBuilder;
  }
}

file static class ExtensionsForIList
{
  public static void AddRange<T>(this IList<T> list, IEnumerable<T> items)
  {
    foreach (var item in items)
    {
      list.Add(item);
    }
  }
}