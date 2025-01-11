using Core.Utils;

namespace Procfiler.Core.CppProcfiler.ShadowStacks;

public class CppShadowStacksImplFromSingleFile(IProcfilerLogger logger, string pathToBinaryStacksFile) : ICppShadowStacks
{
  private readonly object mySync = new();
  private readonly Dictionary<long, long> myManagedThreadsToOffsets = new();

  private bool myIsInitialized;


  public IEnumerable<ICppShadowStack> EnumerateStacks()
  {
    using var fs = PathUtils.OpenReadWithRetryOrThrow(logger, pathToBinaryStacksFile);
    using var br = new BinaryReader(fs);

    foreach (var (_, position) in EnumerateShadowStacksInternal(br))
    {
      if (CppShadowStackImpl.TryCreateShadowStack(logger, pathToBinaryStacksFile, position) is { } shadowStack)
      {
        yield return shadowStack;
      }
    }
  }

  private static IEnumerable<(long ManagedThreadId, long Position)> EnumerateShadowStacksInternal(BinaryReader br)
  {
    var fs = br.BaseStream;
    var position = 0L;
    while (position < fs.Length)
    {
      fs.Seek(position, SeekOrigin.Begin);
      CppShadowStackHelpers.ReadManagedThreadIdAndFramesCount(br, out var threadId, out var framesCount);
      yield return (threadId, position);

      position += CppShadowStackHelpers.CalculateByteLength(framesCount);
    }
  }

  public ICppShadowStack? FindShadowStack(long managedThreadId)
  {
    InitializeThreadIdsToOffsetsIfNeeded();

    if (!myManagedThreadsToOffsets.TryGetValue(managedThreadId, out var offset))
    {
      logger.LogWarning("The shadow stack for {ManagedThreadId} was not found in {Path}", managedThreadId, pathToBinaryStacksFile);
      return null;
    }

    if (CppShadowStackImpl.TryCreateShadowStack(logger, pathToBinaryStacksFile, offset) is not { } shadowStack)
    {
      return null;
    }

    Debug.Assert(shadowStack.ManagedThreadId == managedThreadId);
    return shadowStack;
  }

  private void InitializeThreadIdsToOffsetsIfNeeded()
  {
    if (myIsInitialized) return;

    lock (mySync)
    {
      if (myIsInitialized) return;

      try
      {
        using var fs = PathUtils.OpenReadWithRetryOrThrow(logger, pathToBinaryStacksFile);
        using var br = new BinaryReader(fs);

        foreach (var shadowStack in EnumerateShadowStacksInternal(br))
        {
          myManagedThreadsToOffsets[shadowStack.ManagedThreadId] = shadowStack.Position;
        }

        myIsInitialized = true;
      }
      catch (Exception ex)
      {
        logger.LogError(ex, "Failed to initialize shadow-stacks position");
      }
    }
  }
}