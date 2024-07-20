using System.Text;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Context;
using Procfiler.Core.Collector;
using Procfiler.Utils;
using TestsUtil;

namespace ProcfilerTests.Core;

public class GoldProcessBasedTest : ProcessTestBase
{
  protected void ExecuteTestWithGold(CollectClrEventsFromExeContext context, Func<CollectedEvents, string> testFunc)
  {
    StartProcessAndDoTestWithDefaultContext(context, events =>
    {
      var testValue = testFunc(events).RemoveRn();
      var pathToGoldFile = CreateGoldFilePath();
      if (!File.Exists(pathToGoldFile))
      {
        using var fs = File.CreateText(CreateTmpFilePath());
        fs.WriteAsync(testValue);
        Assert.Fail($"There was not gold file at {pathToGoldFile}");
        return;
      }

      var goldValue = File.ReadAllText(pathToGoldFile).RemoveRn();
      if (goldValue != testValue)
      {
        var sb = new StringBuilder();
        sb.Append("The gold and test value were different:").AppendNewLine()
          .Append("Test value:").AppendNewLine()
          .Append(testValue).AppendNewLine()
          .Append("Gold value:").AppendNewLine()
          .Append(goldValue).AppendNewLine();

        using var fs = File.CreateText(CreateTmpFilePath());
        fs.Write(testValue);

        Assert.Fail(sb.ToString());
      }
    });
  }

  private string CreateGoldFilePath() => CreatePathInternal($"{CreateTestNameForFiles()}.gold");

  private static string CreateTestNameForFiles()
  {
    var test = TestContext.CurrentContext.Test;
    var testName = test.Name;

    if (test.Arguments.FirstOrDefault() is ContextWithSolution dto)
    {
      testName = dto.Solution.Name;
    }

    return testName;
  }

  private string CreatePathInternal(string fileName)
  {
    var folderName = GetType().Name;
    var osPrefix = GetOsFolderOrThrow();
    var directory = Path.Combine(TestPaths.CreatePathToTestData(), "gold", osPrefix, folderName);
    if (!Directory.Exists(directory))
    {
      Directory.CreateDirectory(directory);
    }

    return Path.Combine(directory, fileName);
  }

  private static string GetOsFolderOrThrow()
  {
    if (OperatingSystem.IsWindows()) return "windows";
    if (OperatingSystem.IsLinux()) return "linux";
    if (OperatingSystem.IsMacOS()) return "macos";

    throw new ArgumentOutOfRangeException(Environment.OSVersion.Platform.ToString());
  }

  private string CreateTmpFilePath() => CreatePathInternal($"{CreateTestNameForFiles()}.tmp");
}