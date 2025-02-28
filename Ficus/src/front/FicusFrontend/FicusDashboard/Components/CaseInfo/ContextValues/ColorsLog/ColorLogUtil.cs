using System.Drawing;
using Ficus;

namespace FicusDashboard.Components.CaseInfo.ContextValues.ColorsLog;

public static class ColorLogUtil
{
  public static GrpcColor NewColor(Color color) => new()
  {
    Red = color.R,
    Green = color.G,
    Blue = color.B
  };
}