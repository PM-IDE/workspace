using JetBrains.Lifetimes;

namespace FicusFrontend.Utils;

public class LifetimeDefinitionsByKey<TKey> where TKey : notnull
{
  private readonly Dictionary<TKey, LifetimeDefinition> myDefinitions = [];


  public Lifetime Get(TKey key) => myDefinitions[key].Lifetime;

  public Lifetime CreateNested(TKey key, Lifetime parent)
  {
    var def = parent.CreateNested();
    myDefinitions[key] = def;

    return def.Lifetime;
  }

  public void TerminateAndRemove(TKey key)
  {
    if (myDefinitions.Remove(key, out var definition))
    {
      definition.Terminate();
    }
  }
}