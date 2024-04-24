using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Core.InstrumentalProfiler;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.Processes.Builder;

[AppComponent]
public class DotnetProjectBuilderImpl(
  IProcfilerLogger logger, IDllMethodsPatcher dllMethodsPatcher) : IDotnetProjectBuilder
{
  public BuildResult? TryBuildDotnetProject(ProjectBuildInfo projectBuildInfo)
  {
    var resultNullable = TryBuildDotnetProjectInternal(projectBuildInfo);
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

  private BuildResult? TryBuildDotnetProjectInternal(ProjectBuildInfo projectBuildInfo)
  {
    var (pathToCsproj, tfm, configuration, _, removeTempPath, tempPath, selfContained, args) = projectBuildInfo;
    var projectName = Path.GetFileNameWithoutExtension(pathToCsproj);
    using var _ = new PerformanceCookie($"Building::{projectName}", logger);

    var projectDirectory = Path.GetDirectoryName(pathToCsproj);
    Debug.Assert(projectDirectory is { });

    var artifactsFolderCookie = tempPath switch
    {
      null => CreateTempArtifactsPath(),
      { } => new TempFolderCookie(logger, tempPath)
    };

    var buildConfig = BuildConfigurationExtensions.ToString(configuration);
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
      Arguments =
        $"build {pathToCsproj} -c {buildConfig} -f {tfm} -o {artifactsFolderCookie.FolderPath} --self-contained {selfContained} {args}"
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
    process.OutputDataReceived += (_, args) =>
    {
      outputSb.Append(args.Data);
    };

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

  private TempFolderCookie CreateTempArtifactsPath() => new(logger);
}