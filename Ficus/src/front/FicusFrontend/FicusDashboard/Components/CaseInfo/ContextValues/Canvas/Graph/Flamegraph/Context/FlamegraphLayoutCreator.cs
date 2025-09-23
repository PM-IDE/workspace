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
      if (data.NodePairs.TryGetValue(node, out var pairNode))
      {
        hBlock.InnerBlocks.Add(new SequentialBasicBlock
        {
          NodesSequence = [node]
        });

        var block = new VerticalCompositeBlock();

        var outgoingNodes = data.Edges[node];
        foreach (var (index, outgoingNode) in outgoingNodes.Index())
        {
          if (outgoingNode == pairNode)
          {
            block.InnerBlocks.Add(new EdgeBlock());
          }
          else
          {
            var outgoingNodeEndNode = data.NodePairs.TryGetValue(outgoingNode, out var outgoingNodePair) switch
            {
              true => outgoingNodePair,
              false => pairNode
            };

            block.InnerBlocks.Add(CreateBlockLayoutEndNodeNotInclusive(data, outgoingNode, outgoingNodeEndNode));
          }

          if (index < outgoingNodes.Count - 1)
          {
            block.InnerBlocks.Add(new SeparatorBlock());
          }
        }

        hBlock.InnerBlocks.Add(block);
        node = pairNode;
      }
      else
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

        hBlock.InnerBlocks.Add(new SequentialBasicBlock
        {
          NodesSequence = nodesSequence
        });

        node = currentNode;
      }
    }

    return hBlock;
  }
}