using System.Diagnostics;
using Core.Constants;
using Core.Container;
using Core.Utils;
using Microsoft.Extensions.Logging;

namespace ProcfilerOnline.Core;

public interface IOnlineDotnetProcessLauncher
{
  Process? Launch(string dllPath, string cppProcfilerPath);
}

[AppComponent]
public class OnlineDotnetProcessLauncher(IProcfilerLogger logger) : IOnlineDotnetProcessLauncher
{
  public Process? Launch(string dllPath, string cppProcfilerPath)
  {
    var startInfo = new ProcessStartInfo
    {
      FileName = "dotnet",
      WorkingDirectory = Path.GetDirectoryName(dllPath),
      RedirectStandardOutput = false,
      CreateNoWindow = true,
      Arguments = $"{dllPath}",
      Environment =
      {
        [DotNetEnvs.DefaultDiagnosticPortSuspend] = EnvVarsConstants.True,
        [DotNetEnvs.CoreClrEnableProfiling] = EnvVarsConstants.True,
        [DotNetEnvs.CoreClrProfiler] = "{90684E90-99CE-4C99-A95A-AFE3B9E09E85}",
        [DotNetEnvs.CoreClrProfilerPath] = cppProcfilerPath
      }
    };

    var process = new Process
    {
      StartInfo = startInfo
    };

    if (!process.Start())
    {
      logger.LogError("Failed to start process {Path}", dllPath);
      return null;
    }

    logger.LogInformation("Started process: {Id} {Path} {Arguments}", process.Id, dllPath, startInfo.Arguments);

    return process;
  }
}