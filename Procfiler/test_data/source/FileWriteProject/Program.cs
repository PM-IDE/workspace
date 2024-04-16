namespace FileWriteProject;

internal class Program
{
  public static async Task Main(string[] args)
  {
    var tempFilePath = Path.Combine(Path.GetTempPath(), Path.GetTempFileName());
    using (var writeFs = File.OpenWrite(tempFilePath))
    {
      using var sw = new StreamWriter(writeFs);

      sw.Write("Some content");
    }

    await using (var readFs = File.OpenRead(tempFilePath))
    {
      using var sr = new StreamReader(readFs);

      var text = sr.Read();
    }

    File.Delete(tempFilePath);
  }
}