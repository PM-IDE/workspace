using FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Node;

internal class NodeRenderingUtils
{
  private const string BorderStyle = "solid 1px white;";

  public static string GetBorderStyle(
    FlamegraphRenderingContext context,
    bool isLeftSide,
    bool isLast,
    bool addTopBorder = true
  )
  {
    var firstPart = GetFirstPartOfBorderStyle(context, isLeftSide, addTopBorder);
    var secondPart = GetSecondPartOfBorderStyle(context, isLast);

    return firstPart + secondPart;
  }

  private static string GetSecondPartOfBorderStyle(FlamegraphRenderingContext context, bool isLast) => isLast switch
  {
    true => string.Empty,
    false => context.LeftToRight switch
    {
      true => $"border-bottom: {BorderStyle}",
      false => $"border-right: {BorderStyle}"
    }
  };

  private static string GetFirstPartOfBorderStyle(FlamegraphRenderingContext context, bool isLeftSide, bool addTopBorder) =>
    addTopBorder switch
    {
      false => string.Empty,
      true => context.LeftToRight switch
      {
        true => $"border-{(isLeftSide ? "left" : "right")}: {BorderStyle}",
        false => $"border-{(isLeftSide ? "top" : "bottom")}: {BorderStyle}"
      },
    };
}