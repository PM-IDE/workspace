namespace Procfiler.Core.Collector;

public interface IShadowStacks;

public class EmptyShadowStacks : IShadowStacks
{
  public static EmptyShadowStacks Instance { get; } = new();


  private EmptyShadowStacks()
  {
  }
}