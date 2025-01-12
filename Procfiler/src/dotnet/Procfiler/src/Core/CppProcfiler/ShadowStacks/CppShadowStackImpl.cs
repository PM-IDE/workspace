using Core.Utils;

namespace Procfiler.Core.CppProcfiler.ShadowStacks;

public class CppShadowStackImpl : ICppShadowStack
{
  public static CppShadowStackImpl? TryCreateShadowStack(IProcfilerLogger logger, string filePath, long startPosition)
  {
    using var fs = PathUtils.OpenReadWithRetryOrThrow(logger, filePath);
    using var reader = new BinaryReader(fs);
    if (reader.BaseStream.Length == 0)
    {
      logger.LogWarning("The shadow stacks file is empty for path {Path}", filePath);
      return null;
    }

    reader.BaseStream.Seek(startPosition, SeekOrigin.Begin);

    CppShadowStackHelpers.ReadManagedThreadIdAndFramesCount(reader, out var threadId, out var framesCount);

    return new CppShadowStackImpl(logger, filePath, startPosition, threadId, framesCount);
  }


  private readonly IProcfilerLogger myLogger;
  private readonly string myBinStackFilePath;
  private readonly long myStartPosition;


  public long ManagedThreadId { get; }
  public long FramesCount { get; }


  private CppShadowStackImpl(IProcfilerLogger logger, string filePath, long startPosition, long threadId, long framesCount)
  {
    myLogger = logger;
    myBinStackFilePath = filePath;
    myStartPosition = startPosition;

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