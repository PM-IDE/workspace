namespace Core.Features;

public abstract class Feature(string name)
{
  public string Name { get; } = name;


  public abstract bool IsEnabled();
}