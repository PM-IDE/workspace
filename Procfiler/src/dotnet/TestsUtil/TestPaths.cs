using NUnit.Framework;

namespace TestsUtil;

public static class TestPaths
{
  public static string CreatePathToTestData()
  {
    var dir = Directory.GetParent(AppContext.BaseDirectory)?.Parent?.Parent?.Parent?.Parent?.Parent?.Parent;
    var path = Path.Combine(dir!.FullName, "test_data");

    Assert.That(Directory.Exists(path), Is.True);

    return path;
  }

  public static string CreatePathToSolutionsSource()
  {
    var path = Path.Combine(CreatePathToTestData(), "source");

    Assert.That(Directory.Exists(path), Is.True);

    return path;
  }

  public static string CreatePathToProcfilerLoggerProviderProject()
  {
    var dir = Directory.GetParent(Directory.GetCurrentDirectory())!.Parent!.Parent!.Parent!.FullName;
    var path = Path.Combine(dir, "ProcfilerLoggerProvider", "ProcfilerLoggerProvider.csproj");

    Assert.That(File.Exists(path), Is.True);

    return path;
  }
}