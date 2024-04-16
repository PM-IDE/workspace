using System.Diagnostics;

namespace Bxes.IntegrationTests.BxesImplExecutors;

public interface IBxesImplExecutor
{
  string Name { get; }

  void ConvertToBxes(string xesLogPath, string bxesLogPath);
}

public abstract class ExecutorBase : IBxesImplExecutor
{
  public abstract string Name { get; }


  public void ConvertToBxes(string xesLogPath, string bxesLogPath)
  {
    var process = CreateProcess(xesLogPath, bxesLogPath);
    process.Start();

    var timeout = TimeSpan.FromSeconds(20);
    if (!process.WaitForExit(timeout))
    {
      process.Kill();
      Assert.Fail($"Failed to perform conversion in {timeout}, killing process");
    }

    if (process.ExitCode != 0)
    {
      Assert.Fail($"The conversion {Name} process for {xesLogPath} exited with non-zero exit code {process.ExitCode}");
    }
  }

  protected abstract Process CreateProcess(string xesLogPath, string bxesLogPath);
}
