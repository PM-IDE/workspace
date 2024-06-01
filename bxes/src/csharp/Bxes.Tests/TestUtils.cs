using Bxes.Models.Domain;
using Bxes.Models.System;
using Bxes.Reader;
using Bxes.Tests.Core;
using Bxes.Utils;

namespace Bxes.Tests;

public record struct LogReadWriteTestInputData(string Path, ISystemMetadata SystemMetadata);

public static class TestUtils
{
  public static void ExecuteTestWithTempFile(
    IEventLog log, Func<LogReadWriteTestInputData, EventLogReadResult> testAction)
  {
    ExecuteTestWithPath(
      log, static () => new TempFilePathContainer(), data => ExecuteTestWithLog(log, data, () => testAction(data)));
  }

  public static void ExecuteTestWithTempFolder(
    IEventLog log, Func<LogReadWriteTestInputData, EventLogReadResult> testAction)
  {
    ExecuteTestWithPath(
      log, static () => new TempFolderContainer(), data => ExecuteTestWithLog(log, data, () => testAction(data)));
  }

  private static void ExecuteTestWithPath(
    IEventLog log, Func<IPathContainer> pathCreator, Action<LogReadWriteTestInputData> testAction)
  {
    using var container = pathCreator();
    testAction(new LogReadWriteTestInputData(container.Path, TestLogsProvider.GenerateRandomSystemMetadata(log)));
  }

  public static void ExecuteTestWithLog(
    IEventLog log, LogReadWriteTestInputData data, Func<EventLogReadResult> logProducer)
  {
    var newLog = logProducer();

    AssertUtil.CompareAndSerializeIfFail(log, newLog.EventLog);
    AssertUtil.CompareAndSerializeIfFail(data.SystemMetadata, newLog.SystemMetadata);
  }
}