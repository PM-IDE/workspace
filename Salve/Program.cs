using System.ComponentModel;
using System.Diagnostics;
using Bxes.Models.Domain;
using Bxes.Writer.Stream;
using JetBrains.Annotations;
using Spectre.Console;
using Spectre.Console.Cli;

var app = new CommandApp();
app.Configure(cfg => { cfg.AddCommand<SerializeOutputToBxesCommand>("serialize-to-bxes"); });

app.Run(args);


internal enum ParserKind
{
  Rustc
}

[UsedImplicitly]
internal class SerializeOutputToBxesCommand : Command<SerializeOutputToBxesCommand.Settings>
{
  [UsedImplicitly]
  public class Settings : CommandSettings
  {
    [CommandArgument(0, "<parser>")]
    [Description("The parser which should be used to parse command output")]
    public required ParserKind ParserKind { get; init; }

    [CommandArgument(1, "<o>")]
    [Description("The output path of a bXES file")]
    public required string OutputFilePath { get; init; }

    [CommandArgument(2, "<exec>")]
    [Description("Command executable")]
    public required string Executable { get; init; }

    [CommandOption("--args")]
    [Description("Command arguments")]
    public required string? Arguments { get; init; }

    [CommandOption("--workdir")]
    [Description("Working directory")]
    public required string? WorkingDirectory { get; init; }
  }


  protected override int Execute(CommandContext context, Settings settings, CancellationToken cancellationToken)
  {
    try
    {
      var info = new ProcessStartInfo
      {
        FileName = settings.Executable,
        RedirectStandardOutput = true,
        RedirectStandardError = true,
        Arguments = settings.Arguments,
        CreateNoWindow = true
      };

      var process = new Process
      {
        StartInfo = info
      };

      var processor = LogsProcessorFactory.Create(settings.ParserKind, settings.OutputFilePath);

      try
      {
        // ReSharper disable once AccessToDisposedClosure
        process.OutputDataReceived += (sender, args) => processor.Process(args.Data);
        // ReSharper disable once AccessToDisposedClosure
        process.ErrorDataReceived += (sender, args) => processor.Process(args.Data);

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

internal static class LogsProcessorFactory
{
  public static ILogsProcessor Create(ParserKind parserKind, string outputPath) => parserKind switch
  {
    ParserKind.Rustc => new RustcLogsParser(outputPath),
    _ => throw new ArgumentOutOfRangeException(nameof(parserKind), parserKind, null)
  };
}

internal interface ILogsProcessor : IDisposable
{
  void Process(string? line);
}

internal class RustcLogsParser(string outputPath) : ILogsProcessor
{
  private readonly SingleFileBxesStreamWriterImpl<InMemoryEventImpl> myWriter = new(outputPath, 1);

  public void Process(string? line)
  {
    if (line is { })
    {
      AnsiConsole.WriteLine(line);
    }
  }

  public void Dispose()
  {
    myWriter.Dispose();
  }
}