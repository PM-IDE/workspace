﻿using System.Diagnostics;
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
  public void ExecuteTest()
  {
    var log = TestLogsProvider.CreateSimpleTestLog();

    //todo: remove after #9 is fixed
    foreach (var traceVariant in log.Traces)
    {
      traceVariant.Count = 1;
    }

    var systemMetadata = TestLogsProvider.GenerateRandomSystemMetadata(log);

    using var tempFile = new TempFilePathContainer();
    new SingleFileBxesWriter(systemMetadata).Write(log, tempFile.Path);

    var ficusLogPath = RewriteBxesEventLog(tempFile.Path);

    var readResult = new SingleFileBxesReader().Read(ficusLogPath);

    Assert.Multiple(() =>
    {
      Assert.That(readResult.EventLog, Is.EqualTo(log));
      Assert.That(readResult.SystemMetadata, Is.EqualTo(systemMetadata));
    });
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

    var timeout = TimeSpan.FromSeconds(20000);
    if (!process.WaitForExit(timeout))
    {
      process.Kill();
      Assert.Fail($"The ficus bxes rewrite didnt finish in {timeout}");
    }

    Assert.That(File.Exists(ficusFilePath), Is.True);

    return ficusFilePath;
  }
}