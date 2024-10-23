using System.Diagnostics;
using Bxes.IntegrationTests.XesToBxesTests;
using Bxes.Reader;
using Bxes.Tests.Core;
using Bxes.Utils;
using Bxes.Writer;

namespace Bxes.IntegrationTests.InMemoryLogsTests;

[TestFixture]
public class InMemoryLogsTest
{
  [Test]
  [Repeat(20)]
  public void ExecuteTest()
  {
    var log = RandomLogsGenerator.CreateSimpleLog();

    //todo: remove after #9 is fixed
    foreach (var traceVariant in log.Traces)
    {
      traceVariant.Count = 1;
    }

    var systemMetadata = RandomLogsGenerator.GenerateRandomSystemMetadata(log);

    using var tempFile = new TempFilePathContainer();
    new SingleFileBxesWriter(systemMetadata).Write(log, tempFile.Path);

    var ficusLogPath = RewriteBxesEventLog(tempFile.Path);

    var readResult = new SingleFileBxesReader().Read(ficusLogPath);

    AssertUtil.CompareAndSerializeIfFail(log, readResult.EventLog);
    AssertUtil.CompareAndSerializeIfFail(systemMetadata, readResult.SystemMetadata);
  }

  private static string RewriteBxesEventLog(string bxesLogPath)
  {
    var bxesLogDirectory = Path.GetDirectoryName(bxesLogPath);
    var originalFileName = Path.GetFileNameWithoutExtension(bxesLogPath);
    var ficusFilePath = Path.Combine(bxesLogDirectory, $"{originalFileName}_ficus");

    var process = new Process
    {
      StartInfo = new ProcessStartInfo
      {
        FileName = Environment.GetEnvironmentVariable(EnvVars.PythonPath) ?? "python",
        Arguments = $"{TestDataProvider.FicusBxesToBxesScript} {bxesLogPath} {ficusFilePath} " +
                    $"{Environment.GetEnvironmentVariable(EnvVars.BackendAddr) ?? "localhost:8080"}"
      }
    };

    if (!process.Start())
    {
      Assert.Fail("Failed to start ficus bxes rewrite process");
    }

    var timeout = TimeSpan.FromSeconds(20);
    if (!process.WaitForExit(timeout))
    {
      process.Kill();
      Assert.Fail($"The ficus bxes rewrite didnt finish in {timeout}");
    }

    Assert.That(File.Exists(ficusFilePath), Is.True);

    return ficusFilePath;
  }
}