namespace Core.CppProcfiler;

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
  extension(CppProfilerMode mode)
  {
    public bool IsDisabled() => mode == CppProfilerMode.Disabled;
    public bool IsEnabled() => !mode.IsDisabled();
    public bool IsOnlineSerialization() => mode == CppProfilerMode.PerThreadBinStacksFilesOnline;

    public CppProfilerBinStacksFileMode ToFileMode() => mode switch
    {
      CppProfilerMode.SingleFileBinStack => CppProfilerBinStacksFileMode.SingleFile,
      CppProfilerMode.PerThreadBinStacksFiles => CppProfilerBinStacksFileMode.PerThreadFiles,
      CppProfilerMode.PerThreadBinStacksFilesOnline => CppProfilerBinStacksFileMode.PerThreadFiles,
      _ => throw new ArgumentOutOfRangeException(nameof(mode), mode, null)
    };
  }
}