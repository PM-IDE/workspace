using System.Runtime.InteropServices;
using Core.Container;
using Core.Utils;
using Procfiler.Utils;

namespace Procfiler.Core.Collector;

public interface ITransportCreationWaiter
{
  void WaitUntilTransportIsCreatedOrThrow(int processId);
}

[AppComponent]
public class TransportCreationWaiterImpl(IProcfilerLogger logger) : ITransportCreationWaiter
{
  public void WaitUntilTransportIsCreatedOrThrow(int processId)
  {
    var name = $"{GetType().Name}::{nameof(WaitUntilTransportIsCreatedOrThrow)}::{processId}";
    using var _ = new PerformanceCookie(name, logger);
    var rootPath = RuntimeInformation.IsOSPlatform(OSPlatform.Windows) ? @"\\.\pipe\" : Path.GetTempPath();
    var transportPath = OperatingSystem.IsWindows() switch
    {
      true => $"dotnet-diagnostic-{processId}",
      false => $"dotnet-diagnostic-{processId}-*-socket"
    };

    const int TimeoutMs = 10_000;
    if (!SpinWait.SpinUntil(() => Directory.GetFiles(rootPath, transportPath).Length != 0, TimeoutMs))
    {
      throw new FileNotFoundException($"The transport file {transportPath} was not created after {TimeoutMs}ms");
    }
  }
}