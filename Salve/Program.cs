using System.ComponentModel;
using System.Diagnostics;
using Bxes.Utils;
using JetBrains.Annotations;
using Salve;
using Spectre.Console;
using Spectre.Console.Cli;

var app = new CommandApp();
app.Configure(cfg => { cfg.AddCommand<RustcLogsToBxes>("rustc-logs-to-bxes"); });

app.Run(args);


[UsedImplicitly]
internal class RustcLogsToBxes : Command<RustcLogsToBxes.Settings>
{
  [UsedImplicitly]
  public class Settings : CommandSettings
  {
    [CommandArgument(1, "<o>")]
    [Description("The output path of a bXES file")]
    public required string OutputFilePath { get; init; }

    [CommandOption("--args")]
    [Description("Command arguments")]
    // ReSharper disable once UnassignedGetOnlyAutoProperty
    public string? Arguments { get; init; }

    [CommandOption("--workdir")]
    [Description("Working directory")]
    // ReSharper disable once UnassignedGetOnlyAutoProperty
    public string? WorkingDirectory { get; init; }

    [CommandOption("--group-names-as-event-names")]
    [Description("Use groups names (FQNs) as event names")]
    // ReSharper disable once UnassignedGetOnlyAutoProperty
    public bool UseGroupsAsEventNames { get; init; }
  }


  protected override int Execute(CommandContext context, Settings settings, CancellationToken cancellationToken)
  {
    try
    {
      var directory = Path.GetDirectoryName(settings.OutputFilePath);
      if (!Directory.Exists(directory))
      {
        throw new Exception($"Directory {directory} does not exist");
      }

      PathUtil.EnsureDeleted(settings.OutputFilePath);

      var info = new ProcessStartInfo
      {
        FileName = "rustc",
        RedirectStandardOutput = true,
        RedirectStandardError = true,
        WorkingDirectory = settings.WorkingDirectory,
        Arguments = settings.Arguments,
        CreateNoWindow = true
      };

      var process = new Process
      {
        StartInfo = info
      };

      var processor = new RustcLogsParser(settings.OutputFilePath, settings.UseGroupsAsEventNames);
      processor.Initialize();

      try
      {
        // ReSharper disable once AccessToDisposedClosure
        process.OutputDataReceived += (_, args) => processor.Process(args.Data);
        // ReSharper disable once AccessToDisposedClosure
        process.ErrorDataReceived += (_, args) => processor.Process(args.Data);

        if (!process.Start())
        {
          throw new Exception("Failed to start process");
        }

        process.BeginOutputReadLine();
        process.BeginErrorReadLine();

        process.WaitForExit();
      }
      finally
      {
        processor.Dispose();
      }

      return 0;
    }
    catch (Exception ex)
    {
      AnsiConsole.WriteException(ex);
      return 1;
    }
  }
}