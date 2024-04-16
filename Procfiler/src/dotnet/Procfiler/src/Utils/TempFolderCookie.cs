namespace Procfiler.Utils;

public readonly struct TempFolderCookie : IDisposable
{
  private readonly IProcfilerLogger myLogger;

  public string FolderPath { get; }


  public TempFolderCookie(IProcfilerLogger logger)
  {
    myLogger = logger;
    FolderPath = PathUtils.CreateTempFolderPath();
  }

  public TempFolderCookie(IProcfilerLogger logger, string existingFolder)
  {
    Debug.Assert(Directory.Exists(existingFolder));

    myLogger = logger;
    FolderPath = existingFolder;
  }


  public void Dispose() => PathUtils.DeleteDirectory(FolderPath, myLogger);
}