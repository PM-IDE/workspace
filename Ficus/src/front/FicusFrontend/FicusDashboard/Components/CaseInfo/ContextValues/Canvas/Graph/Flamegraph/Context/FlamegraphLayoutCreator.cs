namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

internal class FlamegraphLayoutCreator
{
  private readonly HashSet<ulong> myAddedNodes = [];


  public HorizontalCompositeBlock Create(FlamegraphContextData data)
  {
    var layout = CreateBlockLayoutEndNodeNotInclusive(data, data.StartNode, data.EndNode);

    AddSingleNodeSequentialBlockIfNeeded(layout, data.EndNode);

    return layout;
  }

  private HorizontalCompositeBlock CreateBlockLayoutEndNodeNotInclusive(
    FlamegraphContextData data, ulong startNode, ulong endNode)
  {
    var hBlock = new HorizontalCompositeBlock();
    var node = startNode;

    while (node != endNode)
    {
      if (data.NodePairs.TryGetValue(node, out var nodePair))
      {
        AddSingleNodeSequentialBlockIfNeeded(hBlock, node);

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

    if (hBlock.InnerBlocks.Count > 0 && hBlock.InnerBlocks.Last() is VerticalCompositeBlock)
    {
      AddSingleNodeSequentialBlockIfNeeded(hBlock, endNode);
    }

    return hBlock;
  }

  private VerticalCompositeBlock CreateVerticalBlock(FlamegraphContextData data, ulong node, NodePair nodePair)
  {
    var block = new VerticalCompositeBlock();

    foreach (var (index, path) in nodePair.Paths.Index())
    {
      if (path.PathNodes.Count > 1)
      {
        if (!path.SyncNode.HasValue)
        {
          throw new Exception("SyncNode was null when path contains several starting nodes");
        }

        var innerVerticalBlock = new VerticalCompositeBlock();

        foreach (var (pathNodesIndex, pathNode) in path.PathNodes.Index())
        {
          innerVerticalBlock.InnerBlocks.Add((pathNode == path.SyncNode.Value) switch
          {
            true => new EdgeBlock
            {
              FromNode = node,
              ToNode = pathNode
            },
            false => CreateBlockLayoutEndNodeNotInclusive(data, pathNode, path.SyncNode.Value)
          });

          if (pathNodesIndex < path.PathNodes.Count - 1)
          {
            innerVerticalBlock.InnerBlocks.Add(new SeparatorBlock());
          }
        }

        var nextHorizontalBlock = CreateBlockLayoutEndNodeNotInclusive(data, path.SyncNode.Value, nodePair.PairedNode);

        var innerHorizontalBlock = new HorizontalCompositeBlock
        {
          InnerBlocks =
          {
            innerVerticalBlock,
            nextHorizontalBlock,
          }
        };

        block.InnerBlocks.Add(innerHorizontalBlock);

        continue;
      }

      var outgoingNode = path.PathNodes[0];
      if (outgoingNode == nodePair.PairedNode)
      {
        block.InnerBlocks.Add(new EdgeBlock
        {
          FromNode = node,
          ToNode = outgoingNode
        });
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

      if (index < nodePair.Paths.Count - 1)
      {
        block.InnerBlocks.Add(new SeparatorBlock());
      }
    }

    return block;
  }

  private (SequentialBasicBlock Block, ulong NextNode) CreateSequentialBlock(FlamegraphContextData data, ulong node, ulong endNode)
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

    var block = CreateSequentialBlock(nodesSequence);

    return (block, currentNode);
  }

  private void AddSingleNodeSequentialBlockIfNeeded(HorizontalCompositeBlock block, ulong node)
  {
    if (!myAddedNodes.Contains(node))
    {
      block.InnerBlocks.Add(CreateSequentialBlock([node]));
    }
  }

  private SequentialBasicBlock CreateSequentialBlock(List<ulong> nodes)
  {
    foreach (var node in nodes)
    {
      myAddedNodes.Add(node);
    }

    return new SequentialBasicBlock
    {
      NodesSequence = nodes.ToList()
    };
  }
}