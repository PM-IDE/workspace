using Ficus;
using Microsoft.AspNetCore.Components;

namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph;

public class FlamegraphContext
{
  private readonly Dictionary<ulong, GrpcGraphNode> myIdsToNodes = [];
  private readonly Dictionary<ulong, List<ulong>> myEdges = [];
  private readonly Dictionary<ulong, List<ulong>> myReversedEdges = [];
  private readonly Dictionary<ulong, ulong> myNodePairs = [];
  private readonly List<List<ulong>> myNodesByLevels;

  public HorizontalCompositeBlock Layout { get; }

  public IReadOnlyDictionary<ulong, GrpcGraphNode> IdsToNodes => myIdsToNodes;
  public IReadOnlyDictionary<ulong, List<ulong>> Edges => myEdges;
  public IReadOnlyDictionary<ulong, List<ulong>> ReversedEdges => myReversedEdges;
  public IDictionary<ulong, ulong> NodePairs => myNodePairs;
  public IReadOnlyList<IReadOnlyList<ulong>> NodesByLevels => myNodesByLevels;


  public FlamegraphContext(GrpcGraph graph)
  {
    foreach (var edge in graph.Edges)
    {
      AddEdge(myEdges, edge.FromNode, edge.ToNode);
      AddEdge(myReversedEdges, edge.ToNode, edge.FromNode);
    }

    foreach (var node in graph.Nodes)
    {
      myIdsToNodes[node.Id] = node;
    }

    var startNode = graph.Nodes.FirstOrDefault(n => !myReversedEdges.ContainsKey(n.Id));
    var rootSequenceNodes = new List<ulong>();

    var currentRootSequenceNode = startNode;
    while (currentRootSequenceNode is { })
    {
      rootSequenceNodes.Add(currentRootSequenceNode.Id);

      if (!myEdges.ContainsKey(currentRootSequenceNode.Id))
      {
        break;
      }

      currentRootSequenceNode = FindNextRootSequenceNode(currentRootSequenceNode);
    }

    if (rootSequenceNodes.Count is 0) throw new Exception("Graph is empty");

    myNodesByLevels = SetNodesByLevels(rootSequenceNodes[0]);
    FindNodesPairs(rootSequenceNodes[0]);

    Layout = CreateBlockLayout(rootSequenceNodes[0], rootSequenceNodes[^1]);
  }

  private void FindNodesPairs(ulong startNode)
  {
    var processed = new HashSet<ulong>();
    var q = new Queue<(ulong, HashSet<int>)>();
    var nextToken = 0;
    var nodesToIssuedTokens = new Dictionary<ulong, (int, int)>();
    var nodesToSeenTokens = new Dictionary<ulong, HashSet<int>>();
    foreach (var key in myIdsToNodes.Keys)
    {
      nodesToSeenTokens[key] = [];
    }

    q.Enqueue((startNode, []));

    while (q.Count > 0)
    {
      var (node, tokens) = q.Dequeue();
      foreach (var token in tokens)
      {
        nodesToSeenTokens[node].Add(token);
      }

      var anyIncomingNodeUnprocessed = false;
      if (myReversedEdges.TryGetValue(node, out var incomingNodes))
      {
        anyIncomingNodeUnprocessed = incomingNodes.Any(incomingNode => !processed.Contains(incomingNode));
      }

      if (anyIncomingNodeUnprocessed)
      {
        continue;
      }

      if (myReversedEdges.TryGetValue(node, out incomingNodes) && incomingNodes.Count > 1)
      {
        foreach (var (issuedNode, issuedTokens) in nodesToIssuedTokens)
        {
          var isPairNode = true;
          for (var t = issuedTokens.Item1; t < issuedTokens.Item2; t++)
          {
            if (!nodesToSeenTokens[node].Contains(t))
            {
              isPairNode = false;
              break;
            }
          }

          if (isPairNode)
          {
            myNodePairs[issuedNode] = node;
            for (var t = issuedTokens.Item1; t < issuedTokens.Item2; t++)
            {
              nodesToSeenTokens[node].Remove(t);
            }

            nodesToIssuedTokens.Remove(issuedNode);
          }
        }
      }

      if (myEdges.TryGetValue(node, out var outgoingNodes))
      {
        if (outgoingNodes.Count == 1)
        {
          q.Enqueue((outgoingNodes[0], tokens));
        }
        else
        {
          nodesToIssuedTokens[node] = (nextToken, nextToken + outgoingNodes.Count);
          foreach (var outgoingNode in outgoingNodes)
          {
            var newSet = new HashSet<int>(nodesToSeenTokens[node]) { nextToken };
            nextToken++;

            q.Enqueue((outgoingNode, newSet));
          }
        }
      }

      processed.Add(node);
    }
  }

  private List<List<ulong>> SetNodesByLevels(ulong startNode)
  {
    var result = new Dictionary<ulong, int>();
    var q = new Queue<(ulong, int)>();
    q.Enqueue((startNode, 0));

    while (q.Count != 0)
    {
      var (node, level) = q.Dequeue();
      if (result.TryGetValue(node, out var prevLevel))
      {
        prevLevel = Math.Max(prevLevel, level);
      }
      else
      {
        prevLevel = level;
      }

      result[node] = prevLevel;

      if (myEdges.TryGetValue(node, out var nextNodes))
      {
        foreach (var nextNode in nextNodes)
        {
          q.Enqueue((nextNode, level + 1));
        }
      }
    }

    return result.GroupBy(p => p.Value).Select(g => g.Select(k => k.Key).ToList()).ToList();
  }

  private void AddEdge(Dictionary<ulong, List<ulong>> map, ulong from, ulong to)
  {
    if (!map.TryGetValue(from, out var toNodes))
    {
      toNodes = [];
      map[from] = toNodes;
    }

    toNodes.Add(to);
  }

  private GrpcGraphNode FindNextRootSequenceNode(GrpcGraphNode node)
  {
    var currentNode = node;
    while (true)
    {
      currentNode = myIdsToNodes[myEdges[currentNode.Id].First()];
      if (currentNode.AdditionalData.Any(d => d.TraceData?.BelongsToRootSequence ?? false))
      {
        return currentNode;
      }
    }
  }

  private HorizontalCompositeBlock CreateBlockLayout(ulong node, ulong endNode)
  {
    var compositeBlock = new HorizontalCompositeBlock();

    while (node != endNode)
    {
      if (myNodePairs.TryGetValue(node, out var pairNode))
      {
        var block = new VerticalCompositeBlock
        {
          StartNode = node,
          EndNode = pairNode
        };

        foreach (var outgoingNode in myEdges[node])
        {
          if (outgoingNode == pairNode)
          {
            block.InnerBlocks.Add(new EdgeBlock());
            continue;
          }

          var outgoingNodeEndNode = myNodePairs.TryGetValue(outgoingNode, out var outgoingNodePair) switch
          {
            true => outgoingNodePair,
            false => pairNode
          };

          block.InnerBlocks.Add(CreateBlockLayout(outgoingNode, outgoingNodeEndNode));
        }

        compositeBlock.InnerBlocks.Add(block);
        node = pairNode;
      }
      else
      {
        var currentNode = node;
        var nodesSequence = new List<ulong>();

        while (currentNode != endNode &&
               !myNodePairs.ContainsKey(currentNode) &&
               myEdges.ContainsKey(currentNode))
        {
          nodesSequence.Add(currentNode);
          currentNode = myEdges[currentNode].First();
        }

        compositeBlock.InnerBlocks.Add(new SequentialBasicBlock
        {
          NodesSequence = nodesSequence
        });

        node = currentNode;
      }
    }

    return compositeBlock;
  }
}