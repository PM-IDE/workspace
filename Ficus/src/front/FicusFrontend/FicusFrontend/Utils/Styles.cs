using System.Drawing;

namespace FicusFrontend.Utils;

public class Style
{
  public required Color BackgroundColor { get; set; }
}

public sealed class StyleKey(string name) : Key<Style>(name);

public sealed class StyleGroup : UserDataHolderBase
{
  public override bool TryGetData<T>(Key<T> key, out T value)
  {
    AssertStyleKey(key);
    return base.TryGetData(key, out value);
  }

  public override void PutData<T>(Key<T> key, T value)
  {
    AssertStyleKey(key);
    base.PutData(key, value);
  }

  private static void AssertStyleKey<T>(Key<T> key)
  {
    if (key is not StyleKey)
    {
      throw new ArgumentOutOfRangeException($"{nameof(key)} must be of type {nameof(StyleKey)}, but was ");
    }
  }
}
