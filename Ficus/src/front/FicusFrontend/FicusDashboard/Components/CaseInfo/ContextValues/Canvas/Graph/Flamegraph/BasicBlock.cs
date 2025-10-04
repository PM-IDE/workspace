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
  public abstract int CalculateMainDim();

  public virtual ulong? GetFirstNode() => null;
  public virtual ulong? GetLastNode() => null;
}

public abstract class CompositeBlockBase : BasicBlock
{
  public List<BasicBlock> InnerBlocks { get; } = [];
}

public class HorizontalCompositeBlock : CompositeBlockBase
{
  public override int CalculateMainDim() => InnerBlocks.Select(b => b.CalculateMainDim()).Max();

  public override ulong? GetFirstNode() => InnerBlocks.Count is 0 ? null : InnerBlocks[0].GetFirstNode();
  public override ulong? GetLastNode() => InnerBlocks.Count is 0 ? null : InnerBlocks[^1].GetLastNode();
}

public class VerticalCompositeBlock : CompositeBlockBase
{
  public required ulong FromNode { get; init; }
  public required ulong ToNode { get; init; }


  public override int CalculateMainDim() => InnerBlocks.Sum(b => b.CalculateMainDim());
}

public class EdgeBlock : BasicBlock
{
  public required ulong FromNode { get; init; }
  public required ulong ToNode { get; init; }


  public override int CalculateMainDim()
  {
    return ModelSizes.EdgeBlock;
  }
}

public class SeparatorBlock : BasicBlock
{
  public override int CalculateMainDim() => ModelSizes.Separtor;
}

public class SequentialBasicBlock : BasicBlock
{
  public required List<ulong> NodesSequence { get; init; }

  public override int CalculateMainDim()
  {
    return ModelSizes.NodeHeight;
  }

  public override ulong? GetFirstNode() => NodesSequence.Count == 0 ? null : NodesSequence[0];
  public override ulong? GetLastNode() => NodesSequence.Count == 0 ? null : NodesSequence[^1];
}