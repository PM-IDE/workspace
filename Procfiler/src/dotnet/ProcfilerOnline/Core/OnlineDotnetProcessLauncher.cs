﻿using System.Diagnostics;
using System.Text.RegularExpressions;
using Core.Constants;
using Core.Container;
using Core.CppProcfiler;
using Core.Utils;
using Microsoft.Extensions.Logging;

namespace ProcfilerOnline.Core;

public interface IOnlineDotnetProcessLauncher
{
  Process? Launch(DotnetProcessLauncherDto launcherDto);
}

public readonly ref struct DotnetProcessLauncherDto
{
  public required string DllPath { get; init; }
  public required string CppProcfilerPath { get; init; }
  public required Regex? MethodsFilterRegex { get; init; }
}

[AppComponent]
public class OnlineDotnetProcessLauncher(IProcfilerLogger logger, IAppExitHandler exitHandler) : IOnlineDotnetProcessLauncher
{
  public Process? Launch(DotnetProcessLauncherDto launcherDto)
  {
    var startInfo = new ProcessStartInfo
    {
      FileName = "dotnet",
      WorkingDirectory = Path.GetDirectoryName(launcherDto.DllPath),
      RedirectStandardOutput = true,
      CreateNoWindow = true,
      Arguments = $"{launcherDto.DllPath}",
      Environment =
      {
        [DotNetEnvs.DefaultDiagnosticPortSuspend] = EnvVarsConstants.True,
        [DotNetEnvs.CoreClrEnableProfiling] = EnvVarsConstants.True,
        [DotNetEnvs.CoreClrProfiler] = "{90684E90-99CE-4C99-A95A-AFE3B9E09E85}",
        [DotNetEnvs.CoreClrProfilerPath] = launcherDto.CppProcfilerPath
      }
    };

    if (launcherDto.MethodsFilterRegex is { } methodsFilterRegex)
    {
      startInfo.EnvironmentVariables[CppProfilerEnvs.MethodsFilterRegex] = methodsFilterRegex.ToString();
    }

    var process = new Process
    {
      StartInfo = startInfo
    };

    process.OutputDataReceived += (_, args) => logger.LogInformation("PROCESS OUTPUT: {ProcessOutput}", args.Data);

    if (!process.Start())
    {
      logger.LogError("Failed to start process {Path}", launcherDto.DllPath);
      return null;
    }

    exitHandler.AddProcess(process);

    process.BeginOutputReadLine();

    logger.LogInformation("Started process: {Id} {Path} {Arguments}", process.Id, launcherDto.DllPath, startInfo.Arguments);

    return process;
  }
}