namespace Core.Utils;

public enum BuildConfiguration
{
  Debug,
  Release
}

public static class BuildConfigurationExtensions
{
  public static string ToString(this BuildConfiguration configuration) => configuration switch
  {
    BuildConfiguration.Debug => "Debug",
    BuildConfiguration.Release => "Release",
    _ => throw new ArgumentOutOfRangeException(nameof(configuration), configuration, null)
  };
}