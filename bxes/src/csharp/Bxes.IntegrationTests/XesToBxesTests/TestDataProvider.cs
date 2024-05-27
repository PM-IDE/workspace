namespace Bxes.IntegrationTests.XesToBxesTests;

public static class TestDataProvider
{
  public static string CommonProjectDirectory { get; } =
    Directory.GetParent(Directory.GetCurrentDirectory())!.Parent!.Parent!.Parent!.Parent!.Parent!.FullName;

  public static string SourceLogDirectory { get; } = Path.Combine(CommonProjectDirectory, "test_data", "conversion_test_logs");

  public static string CSharpExecutable { get; } =
    Path.Combine(CommonProjectDirectory, "src", "csharp", "Bxes.Console", "bin", "Release", "net8.0", "Bxes.Console.dll");

  public static string FicusRustExecutable { get; } =
    Path.Combine(CommonProjectDirectory, "src", "python", "bxes_converter.py");
}