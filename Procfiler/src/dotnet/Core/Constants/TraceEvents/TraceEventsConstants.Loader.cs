namespace Core.Constants.TraceEvents;

public static partial class TraceEventsConstants
{
  public const string LoaderAppDomainLoad = "Loader/AppDomainLoad";
  public const string LoaderAppDomainUnload = "Loader/AppDomainUnload";
  public const string LoaderAppDomainName = "AppDomainName";

  public const string LoaderAssemblyLoad = "Loader/AssemblyLoad";
  public const string LoaderAssemblyUnload = "Loader/AssemblyUnload";
  public const string LoaderAssemblyName = "FullyQualifiedAssemblyName";

  public const string LoaderModuleLoad = "Loader/ModuleLoad";
  public const string LoaderModuleUnload = "Loader/ModuleUnload";
  public const string LoaderILFileName = "ModuleILFileName";

  public const string LoaderDomainModuleLoad = "Loader/DomainModuleLoad";
  public const string LoaderDomainModuleUnload = "Loader/DomainModukeUnload";
  public const string LoaderDomainModueFilePath = "ModuleILPath";
}