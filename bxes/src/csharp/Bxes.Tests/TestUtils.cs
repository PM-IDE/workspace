using Bxes.Models;
using Bxes.Models.Domain;
using Bxes.Utils;

namespace Bxes.Tests;

public static class TestUtils
{
  public static void ExecuteTestWithTempFile(IEventLog log, Func<string, IEventLog> testAction)
  {
    ExecuteTestWithPath(
      static () => new TempFilePathContainer(), file => ExecuteTestWithLog(log, () => testAction(file)));
  }

  public static void ExecuteTestWithTempFolder(IEventLog log, Func<string, IEventLog> testAction)
  {
    ExecuteTestWithPath(
      static () => new TempFolderContainer(), folder => ExecuteTestWithLog(log, () => testAction(folder)));
  }

  private static void ExecuteTestWithPath(Func<IPathContainer> pathCreator, Action<string> testAction)
  {
    using var container = pathCreator();
    testAction(container.Path);
  }

  public static void ExecuteTestWithLog(IEventLog initialLog, Func<IEventLog> logProducer)
  {
    var newLog = logProducer();
    Assert.That(initialLog.Equals(newLog));
  }
}