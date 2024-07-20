using Core.Utils;
using Procfiler.Utils;

namespace Procfiler.Core.CppProcfiler.ShadowStacks;

public class CppShadowStackImpl : ICppShadowStack
{
  private readonly IProcfilerLogger myLogger;
  private readonly string myBinStackFilePath;
  private readonly long myStartPosition;


  public long ManagedThreadId { get; }
  public long FramesCount { get; }


  public CppShadowStackImpl(IProcfilerLogger logger, string filePath, long startPosition)
  {
    myLogger = logger;
    myBinStackFilePath = filePath;
    myStartPosition = startPosition;

    using var fs = PathUtils.OpenReadWithRetryOrThrow(myLogger, myBinStackFilePath);
    using var reader = new BinaryReader(fs);
    reader.BaseStream.Seek(startPosition, SeekOrigin.Begin);

    CppShadowStackHelpers.ReadManagedThreadIdAndFramesCount(reader, out var threadId, out var framesCount);
    ManagedThreadId = threadId;
    FramesCount = framesCount;
  }


  public IEnumerator<FrameInfo> GetEnumerator()
  {
    var fs = PathUtils.OpenReadWithRetryOrThrow(myLogger, myBinStackFilePath);
    var reader = new BinaryReader(fs);

    CppShadowStackHelpers.SeekToPositionAndSkipHeader(reader, myStartPosition);

    return new CppShadowStackEnumerator(reader, FramesCount);
  }

  IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
}