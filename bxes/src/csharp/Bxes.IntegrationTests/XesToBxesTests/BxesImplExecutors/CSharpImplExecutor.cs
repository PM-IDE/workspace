using System.Diagnostics;

namespace Bxes.IntegrationTests.XesToBxesTests.BxesImplExecutors;

public class CSharpImplExecutor : ExecutorBase
{
  public override string Name => "csharp";


  protected override Process CreateProcess(string xesLogPath, string bxesLogPath) => new()
  {
    StartInfo = new ProcessStartInfo
    {
      FileName = "dotnet",
      Arguments = $"{TestDataProvider.CSharpExecutable} " +
                  $"xes-to-bxes " +
                  $"-path {xesLogPath} " +
                  $"-output-path {bxesLogPath} "
    }
  };
}