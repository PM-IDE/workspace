using Core.Builder;
using Core.Collector;
using Core.Container;
using Core.CppProcfiler;
using Core.InstrumentalProfiler;
using Core.Utils;
using Microsoft.Diagnostics.NETCore.Client;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Commands;

namespace ProcfilerOnline.Core;

public interface IClrOnlineEventsProcessor
{
  ISharedEventPipeStreamData? StartProfiling(CollectEventsOnlineBaseContext context);
}

[AppComponent]
public class ClrOnlineEventsProcessor(
  IProcfilerLogger logger,
  IOnlineDotnetProcessLauncher launcher,
  ICppProcfilerLocator locator,
  ITransportCreationWaiter transportCreationWaiter,
  IEventPipeProvidersProvider providersProvider,
  IOnlineEventsProcessor processor,
  IDotnetProjectBuilder dotnetProjectBuilder
) : IClrOnlineEventsProcessor
{
  public ISharedEventPipeStreamData? StartProfiling(CollectEventsOnlineBaseContext context)
  {
    BuildResult? buildResult = null;

    try
    {
      if (context is CollectEventsOnlineFromCsprojContext csprojContext)
      {
        var buildInfo = new ProjectBuildInfo(
          csprojContext.CsprojPath,
          null,
          BuildConfiguration.Release,
          InstrumentationKind.None,
          false,
          ProjectBuildOutputPath.RandomTempPath,
          false,
          null
        );

        if (dotnetProjectBuilder.TryBuildDotnetProject(buildInfo) is not { } result)
        {
          logger.LogError("Failed to build project {CsprojPath}", csprojContext.CsprojPath);
          return null;
        }

        buildResult = result;
      }

      var dllPath = context switch
      {
        CollectEventsOnlineFromCsprojContext => buildResult!.Value.BuiltDllPath,
        CollectEventsOnlineFromDllContext dllContext => dllContext.DllFilePath,
        _ => throw new ArgumentOutOfRangeException(nameof(context))
      };

      var launcherDto = new DotnetProcessLauncherDto
      {
        DllPath = dllPath,
        CppProcfilerPath = locator.FindCppProcfilerPath("CppProcfilerOnline"),
        MethodsFilterRegex = context.MethodsFilterRegex
      };

      var process = launcher.Launch(launcherDto);
      if (process is not { })
      {
        logger.LogError("Failed to start provided .NET application {DllPath}", dllPath);
        return null;
      }

      var client = new DiagnosticsClient(process.Id);
      transportCreationWaiter.WaitUntilTransportIsCreatedOrThrow(process.Id);

      var providers = providersProvider.GetProvidersFor(context.Providers);
      using var session = client.StartEventPipeSession(providers, circularBufferMB: 1024);

      client.ResumeRuntime();
      return processor.Process(session.EventStream, context);
    }
    finally
    {
      if (buildResult is { })
      {
        buildResult.Value.ClearUnderlyingFolder();
        logger.LogInformation("Cleared temp folder {TempFolder} with build outputs", Path.GetDirectoryName(buildResult.Value.BuiltDllPath));
      }
    }
  }
}