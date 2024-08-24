namespace Core.Features;

public class EnvironmentVariableFeature(string featureName, string environmentVarName) : Feature(featureName)
{
  public override bool IsEnabled() => Environment.GetEnvironmentVariable(environmentVarName) switch
  {
    null => false,
    { } value => bool.TryParse(value, out var result) switch
    {
      false => false,
      true => result
    }
  };
}