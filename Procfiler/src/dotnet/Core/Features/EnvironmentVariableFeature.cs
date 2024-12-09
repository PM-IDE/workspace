namespace Core.Features;

public class EnvironmentVariableFeature(string featureName, string environmentVarName, bool defaultValue = false) : Feature(featureName)
{
  public override bool IsEnabled() => Environment.GetEnvironmentVariable(environmentVarName) switch
  {
    null => defaultValue,
    { } value => bool.TryParse(value, out var result) switch
    {
      false => false,
      true => result
    }
  };
}