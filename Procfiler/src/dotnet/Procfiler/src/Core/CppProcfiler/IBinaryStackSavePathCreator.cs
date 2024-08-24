using Core.Builder;
using Core.Container;
using Core.CppProcfiler;
using Core.Utils;

namespace Procfiler.Core.CppProcfiler;

public interface IBinaryStackSavePathCreator
{
  string CreateSavePath(BuildResult buildResult, CppProfilerMode mode);
  string CreateTempSavePath(CppProfilerMode mode);
}

[AppComponent]
public class BinaryStackSavePathCreatorImpl : IBinaryStackSavePathCreator
{
  private const string BinaryStacksFileName = "bstacks.bin";

  public string CreateSavePath(BuildResult buildResult, CppProfilerMode mode)
  {
    switch (mode.ToFileMode())
    {
      case CppProfilerBinStacksFileMode.SingleFile:
      {
        var directory = Path.GetDirectoryName(buildResult.BuiltDllPath);
        Debug.Assert(Directory.Exists(directory));

        return Path.Combine(directory, BinaryStacksFileName);
      }
      case CppProfilerBinStacksFileMode.PerThreadFiles:
      {
        var dirName = Path.GetDirectoryName(buildResult.BuiltDllPath) ??
                      throw new DirectoryNotFoundException(buildResult.BuiltDllPath);

        return AdjustBinStacksSavePath(dirName);
      }
      default:
        throw new ArgumentOutOfRangeException(nameof(mode), mode, null);
    }
  }

  private static string AdjustBinStacksSavePath(string path)
  {
    if (!Path.EndsInDirectorySeparator(path))
    {
      path += Path.DirectorySeparatorChar;
    }

    return path;
  }

  public string CreateTempSavePath(CppProfilerMode mode) => mode.ToFileMode() switch
  {
    CppProfilerBinStacksFileMode.SingleFile => Path.Combine(PathUtils.CreateTempFolderPath(), BinaryStacksFileName),
    CppProfilerBinStacksFileMode.PerThreadFiles => AdjustBinStacksSavePath(PathUtils.CreateTempFolderPath()),
    _ => throw new ArgumentOutOfRangeException(nameof(mode), mode, null)
  };
}