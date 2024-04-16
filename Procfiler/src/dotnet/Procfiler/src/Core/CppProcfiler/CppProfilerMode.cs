namespace Procfiler.Core.CppProcfiler;

public enum CppProfilerMode
{
  Disabled,
  SingleFileBinStack,
  PerThreadBinStacksFiles,
  PerThreadBinStacksFilesOnline
}

public enum CppProfilerBinStacksFileMode
{
  SingleFile,
  PerThreadFiles
}

public static class CppProfilerModeExtensions
{
  public static bool IsDisabled(this CppProfilerMode mode) => mode == CppProfilerMode.Disabled;
  public static bool IsEnabled(this CppProfilerMode mode) => !mode.IsDisabled();
  public static bool IsOnlineSerialization(this CppProfilerMode mode) => mode == CppProfilerMode.PerThreadBinStacksFilesOnline;

  public static CppProfilerBinStacksFileMode ToFileMode(this CppProfilerMode mode) => mode switch
  {
    CppProfilerMode.SingleFileBinStack => CppProfilerBinStacksFileMode.SingleFile,
    CppProfilerMode.PerThreadBinStacksFiles => CppProfilerBinStacksFileMode.PerThreadFiles,
    CppProfilerMode.PerThreadBinStacksFilesOnline => CppProfilerBinStacksFileMode.PerThreadFiles,
    _ => throw new ArgumentOutOfRangeException(nameof(mode), mode, null)
  };
}