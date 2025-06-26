using System.Diagnostics;
using System.Text;
using Core.Constants;
using Core.Container;
using Core.InstrumentalProfiler;
using Core.Utils;
using Microsoft.Extensions.Logging;

namespace Core.Builder;

[AppComponent]
public class DotnetProjectBuilderImpl(
  IProcfilerLogger logger,
  IDllMethodsPatcher dllMethodsPatcher) : IDotnetProjectBuilder
{
  public BuildResult? TryBuildDotnetProject(ProjectBuildInfo projectBuildInfo)
  {
    var resultNullable = TryBuildDotnetProjectWithRetry(projectBuildInfo);
    if (resultNullable is not { } result) return null;

    if (projectBuildInfo.InstrumentationKind is not InstrumentationKind.None)
    {
      var procfilerDirectory = Environment.CurrentDirectory;
      const string ProcfilerEventSourceDllName = $"{InstrumentalProfilerConstants.ProcfilerEventSource}.dll";

      var procfilerEventSourceDll = Directory
        .GetFiles(procfilerDirectory)
        .FirstOrDefault(f => f.EndsWith(ProcfilerEventSourceDllName));

      if (procfilerEventSourceDll is null)
      {
        throw new FileNotFoundException($"Failed to find {InstrumentalProfilerConstants.ProcfilerEventSource}.dll");
      }

      var from = Path.Combine(Environment.CurrentDirectory, procfilerEventSourceDll);
      var buildResultDirName = Path.GetDirectoryName(result.BuiltDllPath);
      Debug.Assert(buildResultDirName is { });

      var to = Path.Combine(buildResultDirName, ProcfilerEventSourceDllName);
      File.Copy(from, to, true);

      dllMethodsPatcher.PatchMethodStartEndAsync(result.BuiltDllPath, projectBuildInfo.InstrumentationKind);
    }

    return result;
  }

  private BuildResult? TryBuildDotnetProjectWithRetry(ProjectBuildInfo projectBuildInfo, uint retriesCount = 10)
  {
    for (var i = 0; i < retriesCount; ++i)
    {
      if (TryBuildDotnetProjectInternal(projectBuildInfo) is { } buildResult)
      {
        return buildResult;
      }

      var secondsUntilNextRetry = Random.Shared.Next(1, 10);

      logger.LogWarning(
        "Failed to build dotnet project {Project}, retrying in {Seconds} secs",
        projectBuildInfo.CsprojPath,
        secondsUntilNextRetry
      );

      Thread.Sleep(TimeSpan.FromSeconds(secondsUntilNextRetry));
    }

    logger.LogError("Failed to build project after {RetriesCount} attempts", retriesCount);

    return null;
  }

  private BuildResult? TryBuildDotnetProjectInternal(ProjectBuildInfo projectBuildInfo)
  {
    var (pathToCsproj, tfm, configuration, _, removeTempPath, tempPath, _, _) = projectBuildInfo;
    var projectName = Path.GetFileNameWithoutExtension(pathToCsproj);
    using var _ = new PerformanceCookie($"Building::{projectName}", logger);

    var projectDirectory = Path.GetDirectoryName(pathToCsproj);
    Debug.Assert(projectDirectory is { });

    if (tempPath is DefaultNetFolder && tfm is null)
    {
      logger.LogWarning("TFM should be specified when building project into default .NET output directory");
    }

    var artifactsFolderCookie = tempPath switch
    {
      DefaultNetFolder => new TempFolderCookie(logger, Path.Combine(projectDirectory, "bin", configuration.ToString(), tfm ?? string.Empty)),
      RandomTempPath => CreateTempArtifactsPath(),
      SpecifiedTempPath { TempFolderPath: var tempFolderPath } => new TempFolderCookie(logger, tempFolderPath),
      _ => throw new ArgumentOutOfRangeException(nameof(tempPath))
    };

    var startInfo = new ProcessStartInfo
    {
      FileName = "dotnet",
      WorkingDirectory = projectDirectory,
      CreateNoWindow = true,
      RedirectStandardOutput = true,
      Environment =
      {
        [DotNetEnvs.DefaultDiagnosticPortSuspend] = EnvVarsConstants.False
      },
      Arguments = BuildArguments(projectBuildInfo, artifactsFolderCookie)
    };

    var process = new Process
    {
      StartInfo = startInfo
    };

    void RemoveArtifactsFolderIfNeeded()
    {
      if (removeTempPath)
      {
        artifactsFolderCookie.Dispose();
      }
    }

    var outputSb = new StringBuilder();
    process.OutputDataReceived += (_, args) => outputSb.Append(args.Data);

    try
    {
      if (!process.Start())
      {
        logger.LogError("Failed to start build process for {PathToCsproj}", pathToCsproj);
        RemoveArtifactsFolderIfNeeded();
        return null;
      }

      process.BeginOutputReadLine();

      var timeout = TimeSpan.FromSeconds(30);
      if (!process.WaitForExit(timeout))
      {
        logger.LogError("Build process didn't exited for {Timeout}ms, killing it", timeout.TotalMilliseconds);
        process.Kill();
        process.WaitForExit();
      }

      if (process.ExitCode != 0)
      {
        logger.LogError("Failed to build project {Path}, {Output}", pathToCsproj, outputSb.ToString());
        RemoveArtifactsFolderIfNeeded();
        return null;
      }
    }
    catch (Exception ex)
    {
      RemoveArtifactsFolderIfNeeded();
      logger.LogError(ex, "Failed to build project {Path}, {Output}", pathToCsproj, outputSb.ToString());
      return null;
    }

    var pathToDll = Path.Combine(artifactsFolderCookie.FolderPath, projectName + ".dll");
    return new BuildResult(artifactsFolderCookie)
    {
      BuiltDllPath = pathToDll
    };
  }

  private static string BuildArguments(ProjectBuildInfo projectBuildInfo, in TempFolderCookie artifactsFolderCookie)
  {
    var buildConfig = BuildConfigurationExtensions.ToString(projectBuildInfo.Configuration);

    var sb = new StringBuilder();
    sb.Append($"build {projectBuildInfo.CsprojPath} ")
      .Append($"-c {buildConfig} ")
      .Append($"-o {artifactsFolderCookie.FolderPath} ")
      .Append($"--self-contained {projectBuildInfo.SelfContained} {projectBuildInfo.AdditionalBuildArgs} ");

    if (projectBuildInfo.Tfm is { } tfm)
    {
      sb.Append($"-f {tfm}");
    }

    return sb.ToString();
  }

  private TempFolderCookie CreateTempArtifactsPath() => new(logger);
}