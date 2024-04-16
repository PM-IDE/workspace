namespace Procfiler.Utils.Container;

public class EventMutatorAttribute(int pass) : AppComponentAttribute
{
  public int Pass { get; } = pass;
}