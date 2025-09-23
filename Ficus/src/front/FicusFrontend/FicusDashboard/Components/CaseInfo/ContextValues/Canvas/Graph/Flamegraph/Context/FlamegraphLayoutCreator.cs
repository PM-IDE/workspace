namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

internal static class FlamegraphLayoutCreator
{
  public static HorizontalCompositeBlock Create(FlamegraphContextData data)
  {
    var layout = CreateBlockLayoutEndNodeNotInclusive(data, data.StartNode, data.EndNode);

    layout.InnerBlocks.Add(new SequentialBasicBlock
    {
      NodesSequence = [data.EndNode]
    });

    return layout;
  }

  private static HorizontalCompositeBlock CreateBlockLayoutEndNodeNotInclusive(
    FlamegraphContextData data, ulong startNode, ulong endNode)
  {
    var hBlock = new HorizontalCompositeBlock();
    var node = startNode;

    while (node != endNode)
    {
      if (data.NodePairs.TryGetValue(node, out var nodePair))
      {
        hBlock.InnerBlocks.Add(new SequentialBasicBlock
        {
          NodesSequence = [node]
        });

        var block = CreateVerticalBlock(data, node, nodePair);

        hBlock.InnerBlocks.Add(block);
        node = nodePair.PairedNode;
      }
      else
      {
        var (block, blockEndNode) = CreateSequentialBlock(data, node, endNode);
        hBlock.InnerBlocks.Add(block);
        node = blockEndNode;
      }
    }

    return hBlock;
  }

  private static VerticalCompositeBlock CreateVerticalBlock(FlamegraphContextData data, ulong node, NodePair nodePair)
  {
    var block = new VerticalCompositeBlock();

    var outgoingNodes = data.Edges[node];
    foreach (var (index, outgoingNode) in outgoingNodes.Index())
    {
      if (outgoingNode == nodePair.PairedNode)
      {
        block.InnerBlocks.Add(new EdgeBlock());
      }
      else
      {
        var outgoingNodeEndNode = data.NodePairs.TryGetValue(outgoingNode, out var outgoingNodePair) switch
        {
          true => outgoingNodePair,
          false => nodePair
        };

        block.InnerBlocks.Add(CreateBlockLayoutEndNodeNotInclusive(data, outgoingNode, outgoingNodeEndNode.PairedNode));
      }

      if (index < outgoingNodes.Count - 1)
      {
        block.InnerBlocks.Add(new SeparatorBlock());
      }
    }

    return block;
  }

  private static (SequentialBasicBlock Block, ulong NextNode) CreateSequentialBlock(
    FlamegraphContextData data, ulong node, ulong endNode)
  {
    var currentNode = node;
    var nodesSequence = new List<ulong>();

    while (currentNode != endNode &&
           !data.NodePairs.ContainsKey(currentNode) &&
           data.Edges.ContainsKey(currentNode))
    {
      nodesSequence.Add(currentNode);
      currentNode = data.Edges[currentNode].First();
    }

    var block = new SequentialBasicBlock
    {
      NodesSequence = nodesSequence
    };

    return (block, currentNode);
  }
}