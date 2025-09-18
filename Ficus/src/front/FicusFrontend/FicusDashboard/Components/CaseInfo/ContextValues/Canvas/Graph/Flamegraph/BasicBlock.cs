using System.Text.Json.Serialization;

namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph;

[JsonDerivedType(typeof(CompositeBasicBlock))]
[JsonDerivedType(typeof(SequentialBasicBlock))]
[JsonDerivedType(typeof(EdgeBlock))]
public abstract class BasicBlock
{
  public abstract int CalculateHeight();
}

public enum Orientation
{
  Vertical,
  Horizontal
}

public class CompositeBasicBlock : BasicBlock
{
  public required ulong StartNode { get; init; }
  public ulong EndNode { get; set; }

  public required Orientation Orientation { get; init; }

  public List<BasicBlock> InnerBlocks { get; } = [];


  public override int CalculateHeight()
  {
    return Orientation switch
    {
      Orientation.Vertical => InnerBlocks.Sum(b => b.CalculateHeight()),
      Orientation.Horizontal => InnerBlocks.Select(b => b.CalculateHeight()).Max(),
      _ => throw new ArgumentOutOfRangeException()
    };
  }
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