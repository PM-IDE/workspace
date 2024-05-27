using System.Diagnostics;

namespace Bxes.IntegrationTests.XesToBxesTests.BxesImplExecutors;

public class RustFicusImplExecutor : ExecutorBase
{
  public override string Name => "ficus";


  protected override Process CreateProcess(string xesLogPath, string bxesLogPath) => new()
  {
    StartInfo = new ProcessStartInfo
    {
      FileName = Environment.GetEnvironmentVariable(EnvVars.PythonPath) ?? "python",
      Arguments = $"{TestDataProvider.FicusRustExecutable} {xesLogPath} {bxesLogPath} " +
                  $"{Environment.GetEnvironmentVariable(EnvVars.BackendAddr)}"
    }
  };
}