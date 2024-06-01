namespace Bxes.Utils;

public interface IPathContainer : IDisposable
{
  string Path { get; }
}

public readonly struct TempFilePathContainer() : IPathContainer
{
  public string Path { get; } = System.IO.Path.GetTempFileName();


  public void Dispose()
  {
    try
    {
      File.Delete(Path);
    }
    catch (Exception ex)
    {
      Console.WriteLine($"Failed to delete temp path {Path}: {ex}");
    }
  }
}

public readonly struct TempFolderContainer() : IPathContainer
{
  public string Path { get; } = Directory.CreateTempSubdirectory().FullName;


  public void Dispose()
  {
    try
    {
      Directory.Delete(Path, true);
    }
    catch (Exception ex)
    {
      Console.WriteLine($"Failed to delete temp directory {Path}: {ex}");
    }
  }
}

public static class PathUtil
{
  public static void EnsureDeleted(string path)
  {
    if (!Path.Exists(path)) return;

    if (File.Exists(path))
    {
      File.Delete(path);
    }

    if (Directory.Exists(path))
    {
      Directory.Delete(path);
    }
  }
}