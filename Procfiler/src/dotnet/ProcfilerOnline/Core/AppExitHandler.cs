using System.Diagnostics;
using Core.Container;
using Core.Utils;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Core.Statistics;

namespace ProcfilerOnline.Core;

public interface IAppExitHandler
{
  void AddProcess(Process process);

  void PerformExitActions();
}

[AppComponent]
public class AppExitHandler : IAppExitHandler
{
  private readonly IProcfilerLogger myLogger;
  private readonly IStatisticsManager myStatisticsManager;
  private readonly HashSet<WeakReference<Process>> myRegisteredProcesses = [];

  public AppExitHandler(IProcfilerLogger logger, IStatisticsManager statisticsManager)
  {
    myLogger = logger;
    myStatisticsManager = statisticsManager;
    Console.CancelKeyPress += (_, args) =>
    {
      HandleAppExit(args);
    };
  }

  public void AddProcess(Process process)
  {
    myRegisteredProcesses.Add(new WeakReference<Process>(process));
  }

  public void PerformExitActions()
  {
    using var _ = new PerformanceCookie($"{GetType().Name}::{nameof(HandleAppExit)}", myLogger);

    myStatisticsManager.Log(myLogger);

    foreach (var processRef in myRegisteredProcesses)
    {
      try
      {
        if (!processRef.TryGetTarget(out var process) || process.HasExited) continue;

        var processId = process.Id + "::" + process.ProcessName + "::" + process.StartInfo.Arguments;
        using var __ = new PerformanceCookie($"Terminating::{processId}", myLogger);

        var timeout = TimeSpan.FromSeconds(10);

        process.Kill(true);
        if (!process.WaitForExit(timeout))
        {
          myLogger.LogInformation("Failed to terminate process in {Timeout}", timeout);
        }
      }
      catch (Exception ex)
      {
        myLogger.LogWarning(ex, "Failed to process process ref");
      }
    }
  }

  private void HandleAppExit(ConsoleCancelEventArgs args) => PerformExitActions();
}