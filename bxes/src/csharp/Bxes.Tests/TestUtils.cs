using Bxes.Models.Domain;
using Bxes.Models.System;
using Bxes.Reader;
using Bxes.Utils;

namespace Bxes.Tests;

public record struct LogReadWriteTestInputData(string Path, ISystemMetadata SystemMetadata);

public static class TestUtils
{
  public static void ExecuteTestWithTempFile(
    IEventLog log, Func<LogReadWriteTestInputData, EventLogReadResult> testAction)
  {
    ExecuteTestWithPath(
      static () => new TempFilePathContainer(), data => ExecuteTestWithLog(log, data, () => testAction(data)));
  }

  public static void ExecuteTestWithTempFolder(
    IEventLog log, Func<LogReadWriteTestInputData, EventLogReadResult> testAction)
  {
    ExecuteTestWithPath(
      static () => new TempFolderContainer(), data => ExecuteTestWithLog(log, data, () => testAction(data)));
  }

  private static void ExecuteTestWithPath(
    Func<IPathContainer> pathCreator, Action<LogReadWriteTestInputData> testAction)
  {
    using var container = pathCreator();
    testAction(new LogReadWriteTestInputData(container.Path, TestLogsProvider.GenerateRandomSystemMetadata()));
  }

  public static void ExecuteTestWithLog(
    IEventLog log, LogReadWriteTestInputData data, Func<EventLogReadResult> logProducer)
  {
    var newLog = logProducer();
    Assert.That(log.Equals(newLog.EventLog));
    Assert.That(data.SystemMetadata.Equals(newLog.SystemMetadata));
  }
}