using Core.Container;
using Core.Utils;

namespace Procfiler.Core.CppProcfiler;

public interface ICppProcfilerLocator
{
  string FindCppProcfilerPath();
}

[AppComponent]
public class CppProcfilerLocatorImpl(IProcfilerLogger logger) : ICppProcfilerLocator
{
  public string FindCppProcfilerPath()
  {
    var procfilerAssemblyLocation = Path.GetDirectoryName(GetType().Assembly.Location);
    if (procfilerAssemblyLocation is null)
    {
      logger.LogError("The Procfiler.dll has no path: {Path}", procfilerAssemblyLocation);
      throw new FileNotFoundException();
    }

    var path = Path.Combine(procfilerAssemblyLocation, "CppProcfiler.dll");
    if (!File.Exists(path))
    {
      logger.LogError("The CppProcfiler.dll does not exist here: {Path}", path);
      throw new FileNotFoundException();
    }

    logger.LogInformation("The cpp Procfiler is located at {Path}", path);
    return path;
  }
}