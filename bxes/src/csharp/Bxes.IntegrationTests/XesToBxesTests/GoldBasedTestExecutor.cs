using Bxes.IntegrationTests.XesToBxesTests.BxesImplExecutors;
using Bxes.Reader;

namespace Bxes.IntegrationTests.XesToBxesTests;

public static class GoldBasedTestExecutor
{
  public static void Execute(IEnumerable<IBxesImplExecutor> executors, string xesLogPath)
  {
    var tempPath = Directory.CreateTempSubdirectory().FullName;

    try
    {
      foreach (var executor in executors)
      {
        var bxesLogPath = Path.Combine(tempPath, $"{executor.Name}.bxes");
        executor.ConvertToBxes(xesLogPath, bxesLogPath);
      }

      var files = Directory.EnumerateFiles(tempPath).ToList();
      var goldLog = new SingleFileBxesReader().Read(files[0]);

      foreach (var filePath in files[1..])
      {
        var currentLog = new SingleFileBxesReader().Read(filePath);

        if (!currentLog.Equals(goldLog))
        {
          Assert.Fail($"The log from execution {Path.GetFileNameWithoutExtension(filePath)}, " +
                      $"differs from gold log from {Path.GetFileNameWithoutExtension(files[0])}, " +
                      $"original log: {Path.GetFileNameWithoutExtension(xesLogPath)}");
        }
      }
    }
    finally
    {
      Directory.Delete(tempPath, true);
    }
  }
}