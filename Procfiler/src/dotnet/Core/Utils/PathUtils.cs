using Microsoft.Extensions.Logging;

namespace Core.Utils;

public static class PathUtils
{
  public static void ThrowIfNotExists(string path)
  {
    if (!Path.Exists(path))
    {
      throw new FileNotFoundException(path);
    }
  }

  public static void CheckIfDirectoryOrThrow(string path)
  {
    if (!File.GetAttributes(path).HasFlag(FileAttributes.Directory))
      throw new DirectoryNotFoundException();
  }

  public static void EnsureEmptyDirectory(string path, IProcfilerLogger logger)
  {
    try
    {
      EnsureEmptyDirectoryOrThrow(path);
    }
    catch (Exception ex)
    {
      logger.LogError(ex.Message);
    }
  }

  public static void EnsureEmptyDirectoryOrThrow(string path)
  {
    if (Directory.Exists(path))
    {
      Directory.Delete(path, true);
    }

    Directory.CreateDirectory(path);
  }

  public static void ClearPathIfExists(string path, IProcfilerLogger logger)
  {
    try
    {
      if (Path.Exists(path) && (File.GetAttributes(path) & FileAttributes.Directory) != 0)
      {
        EnsureEmptyDirectoryOrThrow(path);
      }
      else
      {
        if (File.Exists(path))
        {
          File.Delete(path);
        }
      }
    }
    catch (Exception ex)
    {
      logger.LogError(ex, ex.Message);
    }
  }

  public static void DeleteDirectory(string path, IProcfilerLogger logger)
  {
    if (!Directory.Exists(path)) return;

    try
    {
      Directory.Delete(path, true);
    }
    catch (Exception ex)
    {
      logger.LogError(ex, "Failed to delete directory {Path}", path);
    }
  }

  public static string CreateTempFilePath() => Path.Combine(Path.GetTempPath(), Path.GetTempFileName());

  public static string CreateTempFolderPath() => Directory.CreateTempSubdirectory().FullName;

  public static FileStream OpenReadWithRetryOrThrow(
    IProcfilerLogger logger, string path, int retryCount = 5, int timeoutMs = 500)
  {
    for (var i = 0; i < retryCount; ++i)
    {
      try
      {
        return File.OpenRead(path);
      }
      catch (IOException)
      {
        const string Message = "Failed to open {Path}, retry number {Index}/{Total}, timeout = {Timeout}";
        logger.LogTrace(Message, path, i + 1, retryCount, timeoutMs);
        Thread.Sleep(timeoutMs);
      }
    }

    throw new IOException($"Failed to open {path} after {retryCount} retries with timeout {timeoutMs}");
  }

  public static async Task<FileStream> OpenReadWithRetryOrThrowAsync(
    IProcfilerLogger logger, string path, int retryCount = 5, int timeoutMs = 500)
  {
    for (var i = 0; i < retryCount; ++i)
    {
      try
      {
        return File.OpenRead(path);
      }
      catch (IOException)
      {
        const string Message = "Failed to open {Path}, retry number {Index}/{Total}, timeout = {Timeout}";
        logger.LogTrace(Message, path, i + 1, retryCount, timeoutMs);
        await Task.Delay(timeoutMs);
      }
    }

    throw new IOException($"Failed to open {path} after {retryCount} retries with timeout {timeoutMs}");
  }
}