using Core.Container;
using Core.CppProcfiler;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.CppProcfiler.ShadowStacks;

namespace Procfiler.Core.CppProcfiler;

public class FrameInfo
{
  public long QpcTimeStamp { get; set; }
  public long FunctionId { get; set; }
  public bool IsStart { get; set; }


  public string Serialize(SessionGlobalData? globalData)
  {
    var fqnOrId = globalData?.FindMethodDetails(FunctionId)?.Fqn switch
    {
      { } fqn => fqn,
      _ => FunctionId.ToString()
    };

    var startOrEnd = IsStart ? "start" : " end ";
    return $"[{QpcTimeStamp}] [{startOrEnd}] {fqnOrId}";
  }
}

public interface IBinaryShadowStacksReader
{
  IShadowStacks ReadStackEvents(string path, CppProfilerMode mode);
}

[AppComponent]
public class BinaryShadowStacksReaderImpl(IProcfilerLogger logger) : IBinaryShadowStacksReader
{
  public IShadowStacks ReadStackEvents(string path, CppProfilerMode mode) => mode.ToFileMode() switch
  {
    CppProfilerBinStacksFileMode.SingleFile => new CppShadowStacksImplFromSingleFile(logger, path),
    CppProfilerBinStacksFileMode.PerThreadFiles => new CppShadowStackFromSeveralFiles(logger, path),
    _ => throw new ArgumentOutOfRangeException(nameof(mode), mode, null)
  };
}