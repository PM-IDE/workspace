using System.Drawing;

namespace FicusFrontend.Utils;

public class Style
{
  public required Color BackgroundColor { get; set; }
}

public sealed class StyleKey(string name) : Key<Style>(name);

public sealed class StyleGroup : UserDataHolderBase
{
  public bool TryGetData(StyleKey key, out Style value) => base.TryGetData(key, out value);
  public void PutData(StyleKey key, Style value) => base.PutData(key, value);
}
