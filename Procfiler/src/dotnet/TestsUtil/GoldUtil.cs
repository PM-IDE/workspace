using System.Text;
using Core.Utils;
using NUnit.Framework;

namespace TestsUtil;

public static class GoldUtil
{
  public static void ExecuteGoldTest(string testValue, string folderName, Func<TestContext.TestAdapter, string> testNameExtractor)
  {
    var pathToGoldFile = CreateGoldFilePath(folderName, testNameExtractor);

    if (!File.Exists(pathToGoldFile))
    {
      using var fs = File.CreateText(CreateTmpFilePath(folderName, testNameExtractor));
      fs.Write(testValue);
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

      using var fs = File.CreateText(CreateTmpFilePath(folderName, testNameExtractor));
      fs.Write(testValue);

      Assert.Fail(sb.ToString());
    }
  }

  public static string CreateGoldFilePath(string folderName, Func<TestContext.TestAdapter, string> testNameExtractor) =>
    CreatePathInternal(folderName, $"{CreateTestNameForFiles(testNameExtractor)}.gold");

  private static string CreateTestNameForFiles(Func<TestContext.TestAdapter, string> testNameExtractor)
  {
    var test = TestContext.CurrentContext.Test;
    return testNameExtractor(test);
  }

  private static string CreatePathInternal(string folderName, string fileName)
  {
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

  public static string CreateTmpFilePath(string folderName, Func<TestContext.TestAdapter, string> testNameExtractor) =>
    CreatePathInternal(folderName, $"{CreateTestNameForFiles(testNameExtractor)}.tmp");
}