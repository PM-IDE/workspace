using Core.Collector;
using Core.Container;
using Core.CppProcfiler;
using Core.Utils;
using Microsoft.Diagnostics.NETCore.Client;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Commands;

namespace ProcfilerOnline.Core;

public interface IClrOnlineEventsProcessor
{
  ISharedEventPipeStreamData? StartProfiling(CollectEventsOnlineContext context);
}

[AppComponent]
public class ClrOnlineEventsProcessor(
  IProcfilerLogger logger,
  IOnlineDotnetProcessLauncher launcher,
  ICppProcfilerLocator locator,
  ITransportCreationWaiter transportCreationWaiter,
  IEventPipeProvidersProvider providersProvider,
  IOnlineEventsProcessor processor
) : IClrOnlineEventsProcessor
{
  public ISharedEventPipeStreamData? StartProfiling(CollectEventsOnlineContext context)
  {
    var launcherDto = new DotnetProcessLauncherDto
    {
      DllPath = context.DllFilePath,
      CppProcfilerPath = locator.FindCppProcfilerPath("CppProcfilerOnline"),
      MethodsFilterRegex = context.MethodsFilterRegex
    };

    var process = launcher.Launch(launcherDto);
    if (process is not { })
    {
      logger.LogError("Failed to start provided .NET application {DllPath}", context.DllFilePath);
      return null;
    }

    var client = new DiagnosticsClient(process.Id);
    transportCreationWaiter.WaitUntilTransportIsCreatedOrThrow(process.Id);

    var providers = providersProvider.GetProvidersFor(ProvidersCategoryKind.All);
    using var session = client.StartEventPipeSession(providers, circularBufferMB: 1024);

    client.ResumeRuntime();
    return processor.Process(session.EventStream, context);
  }
}