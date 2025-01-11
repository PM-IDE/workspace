using Core.Utils;

namespace Procfiler.Core.CppProcfiler.ShadowStacks;

public partial class CppShadowStackFromSeveralFiles(IProcfilerLogger logger, string pathToBinaryStacksFolder) : ICppShadowStacks
{
  [GeneratedRegex(@"binstack_[0-9]+\.bin")]
  private static partial Regex BinStacksFileRegex();


  public IEnumerable<ICppShadowStack> EnumerateStacks()
  {
    if (!Directory.Exists(pathToBinaryStacksFolder))
    {
      logger.LogError("The bin stacks directory {Path} does not exist", pathToBinaryStacksFolder);
      yield break;
    }

    var binStacksFileRegex = BinStacksFileRegex();
    var binStacksFiles = Directory.GetFiles(pathToBinaryStacksFolder)
      .Select(Path.GetFileName)
      .Where(file => file is { } && binStacksFileRegex.IsMatch(file));

    foreach (var binStacksFile in binStacksFiles)
    {
      var path = Path.Join(pathToBinaryStacksFolder, binStacksFile);

      if (CppShadowStackImpl.TryCreateShadowStack(logger, path, 0) is { } shadowStack)
      {
        yield return shadowStack;
      }
    }
  }

  public ICppShadowStack? FindShadowStack(long managedThreadId)
  {
    var threadBinStacksFilePath = Path.Join(pathToBinaryStacksFolder, $"binstack_{managedThreadId}.bin");
    if (!File.Exists(threadBinStacksFilePath)) return null;

    return CppShadowStackImpl.TryCreateShadowStack(logger, threadBinStacksFilePath, 0);
  }
}