namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph;

public static class ModelSizes
{
  public const int Base = 1;
  public const int Separtor = Base;
  public const int NodeHeight = 6 * Base;
  public const int EdgeBlock = NodeHeight;
}

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

  public ulong? GetFirstNode()
  {
    if (InnerBlocks.Count is 0) return null;

    return InnerBlocks[0] switch
    {
      VerticalCompositeBlock => null,
      SequentialBasicBlock sequentialBlock => sequentialBlock.GetFirstNode(),
      HorizontalCompositeBlock horizontalBock => horizontalBock.GetFirstNode(),
      _ => null
    };
  }
}

public class VerticalCompositeBlock : CompositeBlockBase
{
  public required ulong FromNode { get; init; }
  public required ulong ToNode { get; init; }


  public override int CalculateHeight() => InnerBlocks.Sum(b => b.CalculateHeight());
}

public class EdgeBlock : BasicBlock
{
  public required ulong FromNode { get; init; }
  public required ulong ToNode { get; init; }


  public override int CalculateHeight()
  {
    return ModelSizes.EdgeBlock;
  }
}

public class SeparatorBlock : BasicBlock
{
  public override int CalculateHeight() => ModelSizes.Separtor;
}

public class SequentialBasicBlock : BasicBlock
{
  public required List<ulong> NodesSequence { get; init; }

  public override int CalculateHeight()
  {
    return ModelSizes.NodeHeight;
  }

  public ulong? GetFirstNode() => NodesSequence.Count == 0 ? null : NodesSequence[0];
}