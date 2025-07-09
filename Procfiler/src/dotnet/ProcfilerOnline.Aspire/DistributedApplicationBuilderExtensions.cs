using Aspire.Hosting;
using Aspire.Hosting.ApplicationModel;

namespace ProcfilerOnline.Aspire;

public static class DistributedApplicationBuilderExtensions
{
  public class ProcfilerExecutableResource(string name, string command, string workingDirectory)
    : ExecutableResource(name, command, workingDirectory), IResourceWithServiceDiscovery;

  public class ProcfilerSettings
  {
    public string? TargetMethodsRegex { get; set; }
    public string? MethodsFilterRegex { get; set; }

    public bool ProduceEventsToKafka { get; set; } = true;
    public bool ProduceBxesKafkaEvents { get; set; } = true;
    public bool ProduceGcEvents { get; set; } = true;

    public string? TopicName { get; set; }
    public string? BootstrapServers { get; set; }
  }

  public static IResourceBuilder<ProcfilerExecutableResource> AddLocalProcfilerExecutable<TProject>(
    this IDistributedApplicationBuilder builder,
    string name,
    string localProcfilerExecutablePath,
    Action<ProcfilerSettings> configure
  ) where TProject : IProjectMetadata, new()
  {
    var projectPath = new TProject().ProjectPath;
    var projectName = Path.GetFileNameWithoutExtension(projectPath);

    var settings = new ProcfilerSettings();
    configure(settings);

    var projectResource = builder
      .AddProject<TProject>(name)
      .WithEnvironment("ProduceEventsToKafka", settings.ProduceEventsToKafka.ToString())
      .WithEnvironment("ProduceBxesKafkaEvents", settings.ProduceBxesKafkaEvents.ToString())
      .WithEnvironment("ProduceGcEvents", settings.ProduceGcEvents.ToString())
      .WithEnvironment("OnlineProcfilerSettings__KafkaSettings__TopicName", settings.TopicName)
      .WithEnvironment("OnlineProcfilerSettings__KafkaSettings__BootstrapServers", settings.BootstrapServers);

    builder.Resources.Remove(projectResource.Resource);

    var procfilerExecutableResource = new ProcfilerExecutableResource(
      name, localProcfilerExecutablePath, Path.GetDirectoryName(projectPath)!);

    var resourceBuilder = builder
      .AddResource(procfilerExecutableResource)
      .WithArgs(context => context.Args.AddRange([
        "collect-online",
        "-csproj",
        projectPath,
        "--target-methods-regex",
        settings.TargetMethodsRegex ?? projectName,
        "--methods-filter-regex",
        settings.MethodsFilterRegex ?? projectName
      ]));

    foreach (var resourceAnnotation in projectResource.Resource.Annotations)
    {
      resourceBuilder.WithAnnotation(resourceAnnotation);
    }

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