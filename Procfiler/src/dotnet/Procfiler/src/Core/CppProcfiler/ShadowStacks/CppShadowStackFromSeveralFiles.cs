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
      yield return new CppShadowStackImpl(logger, Path.Join(pathToBinaryStacksFolder, binStacksFile), 0);
    }
  }

  public ICppShadowStack? FindShadowStack(long managedThreadId)
  {
    var threadBinStacksFilePath = Path.Join(pathToBinaryStacksFolder, $"binstack_{managedThreadId}.bin");
    if (!File.Exists(threadBinStacksFilePath)) return null;

    return new CppShadowStackImpl(logger, threadBinStacksFilePath, 0);
  }
}