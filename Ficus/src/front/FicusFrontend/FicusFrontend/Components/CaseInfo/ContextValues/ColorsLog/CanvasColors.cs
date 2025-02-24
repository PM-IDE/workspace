using System.Drawing;
using Ficus;

namespace FicusFrontend.Components.CaseInfo.ContextValues.ColorsLog;

public class CanvasColors
{
  public static CanvasColors Instance { get; } = new();


  public GrpcColor Background { get; } = ColorLogUtil.NewColor(Color.FromArgb(37, 37, 37));
  public GrpcColor Axis { get; } = ColorLogUtil.NewColor(Color.FromArgb(80, 80, 80));


  private CanvasColors()
  {
  }
}