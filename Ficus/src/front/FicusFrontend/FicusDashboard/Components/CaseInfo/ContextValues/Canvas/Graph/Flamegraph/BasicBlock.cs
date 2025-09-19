using System.Text.Json.Serialization;

namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph;

public abstract class BasicBlock
{
  public abstract int CalculateHeight();
}

public abstract class CompositeBlockBase : BasicBlock
{
  public List<BasicBlock> InnerBlocks { get; } = [];
}

public class HorizontalCompositeBlock : CompositeBlockBase
{
  public override int CalculateHeight() => InnerBlocks.Select(b => b.CalculateHeight()).Max();
}

public class VerticalCompositeBlock : CompositeBlockBase
{
  public required ulong StartNode { get; init; }
  public ulong EndNode { get; set; }


  public override int CalculateHeight() => InnerBlocks.Sum(b => b.CalculateHeight());
}

public class EdgeBlock : BasicBlock
{
  public override int CalculateHeight()
  {
    return 1;
  }
}

public class SequentialBasicBlock : BasicBlock
{
  public required List<ulong> NodesSequence { get; init; }

  public override int CalculateHeight()
  {
    return 1;
  }
}