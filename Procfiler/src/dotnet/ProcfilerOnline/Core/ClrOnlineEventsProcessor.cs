using Core.Collector;
using Core.Container;
using Core.CppProcfiler;
using Core.EventsProcessing.Mutators.Core;
using Core.Utils;
using Microsoft.Diagnostics.NETCore.Client;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Commands;
using ProcfilerOnline.Core.Processors;
using ProcfilerOnline.Core.Statistics;

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
  IEventPipeProvidersProvider providersProvider,
  IEnumerable<ITraceEventProcessor> processors,
  IEnumerable<ISingleEventMutator> singleEventMutators,
  IStatisticsManager statisticsManager
) : IOnlineEventsProcessor
{
  public void StartProfiling(CollectEventsOnlineContext context)
  {
    var launcherDto = new DotnetProcessLauncherDto
    {
      DllPath = context.DllFilePath,
      CppProcfilerPath = locator.FindCppProcfilerPath("CppProcfilerOnline"),
      MethodsFilterRegex = context.MethodsFilterRegex,
    };

    var process = launcher.Launch(launcherDto);
    if (process is not { })
    {
      logger.LogError("Failed to start provided .NET application {DllPath}", context.DllFilePath);
      return;
    }

    var client = new DiagnosticsClient(process.Id);
    transportCreationWaiter.WaitUntilTransportIsCreatedOrThrow(process.Id);

    var providers = providersProvider.GetProvidersFor(ProvidersCategoryKind.CppProcfilerMethodsAndTasks);
    var session = client.StartEventPipeSession(providers, requestRundown: false, circularBufferMB: 1024);

    client.ResumeRuntime();

    var processor = new OnlineEventsProcessorImpl(logger, processors, context, singleEventMutators, statisticsManager);
    processor.Process(session.EventStream);
  }
}