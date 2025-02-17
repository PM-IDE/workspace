using System.Drawing;

namespace FicusFrontend.Components.CaseInfo.ContextValues.ColorsLog;

public class CanvasColors
{
  public static CanvasColors Instance { get; } = new();


  public Color Background { get; } = Color.FromArgb(37, 37, 37);
  public Color Axis { get; } = Color.FromArgb(42, 42, 42);


  private CanvasColors()
  {
  }
}