using Core.Collector;
using Core.Container;
using Core.CppProcfiler;
using Core.Utils;
using Microsoft.Diagnostics.NETCore.Client;
using Microsoft.Diagnostics.Tracing;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Commands;

namespace ProcfilerOnline.Core;

public interface IOnlineEventsProcessor
{
  void StartProfiling(CollectEventsOnlineContext context);
}

[AppComponent]
public class ClrOnlineEventsProcessor(
  IProcfilerLogger logger,
  IOnlineDotnetProcessLauncher launcher,
  ICppProcfilerLocator locator,
  ITransportCreationWaiter transportCreationWaiter,
  IEventPipeProvidersProvider providersProvider
  ) : IOnlineEventsProcessor
{
  public void StartProfiling(CollectEventsOnlineContext context)
  {
    var process = launcher.Launch(context.DllFilePath, locator.FindCppProcfilerPath("CppProcfilerOnline"));
    if (process is not { })
    {
      logger.LogError("Failed to start provided .NET application {DllPath}", context.DllFilePath);
      return;
    }

    var client = new DiagnosticsClient(process.Id);
    transportCreationWaiter.WaitUntilTransportIsCreatedOrThrow(process.Id);

    var providers = providersProvider.GetProvidersFor(ProvidersCategoryKind.CppProcfiler);
    var session = client.StartEventPipeSession(providers, requestRundown: false, circularBufferMB: 2048);
    var eventPipeEventSource = CreateEventPipeSource(session);

    client.ResumeRuntime();

    eventPipeEventSource.Process();
  }

  private EventPipeEventSource CreateEventPipeSource(EventPipeSession session)
  {
    var source = new EventPipeEventSource(session.EventStream);
    source.Dynamic.All += traceEvent =>
    {
      logger.LogInformation(traceEvent.EventName);
    };

    return source;
  }
}