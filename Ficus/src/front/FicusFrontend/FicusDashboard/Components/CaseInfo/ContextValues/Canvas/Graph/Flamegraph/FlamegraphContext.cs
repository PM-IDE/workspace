using System.Collections;
using Ficus;

namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph;

public class FlamegraphContext
{
  private readonly Dictionary<ulong, GrpcGraphNode> myIdsToNodes = [];
  private readonly Dictionary<ulong, List<ulong>> myEdges = [];
  private readonly Dictionary<ulong, List<ulong>> myReversedEdges = [];
  private readonly Dictionary<ulong, ulong> myNodePairs = [];

  public HorizontalCompositeBlock Layout { get; }

  public IReadOnlyDictionary<ulong, GrpcGraphNode> IdsToNodes => myIdsToNodes;
  public IReadOnlyDictionary<ulong, List<ulong>> Edges => myEdges;
  public IReadOnlyDictionary<ulong, List<ulong>> ReversedEdges => myReversedEdges;
  public IDictionary<ulong, ulong> NodePairs => myNodePairs;


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
    if (startNode is null) throw new Exception("Graph does not contain a start node");

    var endNode = graph.Nodes.FirstOrDefault(n => !myEdges.ContainsKey(n.Id));
    if (endNode is null) throw new Exception("Graph does not contain an end node");

    FindNodesPairs(startNode.Id);

    Layout = CreateLayout(startNode.Id, endNode.Id);
  }

  private class IssuedTokens(int startToken, int rightBorder)
  {
    private int myNextIndex;


    private readonly int?[] myMergedTokens = new int?[rightBorder - startToken];


    public bool FoundPairNode { get; set; }
    public int StartToken { get; } = startToken;
    public int RightBorder { get; } = rightBorder;


    public void AddTokensGroup(List<int> tokens)
    {
      if (tokens.Count < 2) return;

      foreach (var index in tokens.Select(token => token - StartToken))
      {
        if (myMergedTokens[index] is { })
        {
          throw new Exception("Some of the tokens already merged, can not merge twice");
        }

        myMergedTokens[index] = myNextIndex;
      }

      myNextIndex++;
    }

    public List<List<ulong>> GroupOutgoingNodesByPaths(List<ulong> outgoingNodes)
    {
      if (myNextIndex is 0)
      {
        return outgoingNodes.Select(n => new List<ulong> { n }).ToList();
      }

      var result = new List<List<ulong>>();
      var groupsLists = Enumerable.Range(0, myNextIndex).Select(_ => new List<ulong>()).ToArray();
      foreach (var (mergedToken, node) in myMergedTokens.Zip(outgoingNodes))
      {
        if (mergedToken is not { } token)
        {
          result.Add([node]);
          continue;
        }

        groupsLists[token].Add(node);
      }

      result.AddRange(groupsLists);

      return result;
    }
  }

  private void FindNodesPairs(ulong startNode)
  {
    var processed = new HashSet<ulong>();
    var q = new Queue<(ulong, HashSet<int>)>();
    var nextToken = 0;
    var nodesToIssuedTokens = new Dictionary<ulong, IssuedTokens>();
    var nodesToSeenTokens = new Dictionary<ulong, HashSet<int>>();
    var nodesToQueuedHashSets = new Dictionary<ulong, HashSet<int>>();

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

      nodesToQueuedHashSets.Remove(node);
      if (myReversedEdges.TryGetValue(node, out incomingNodes) && incomingNodes.Count > 1)
      {
        foreach (var (issuedNode, issuedTokens) in nodesToIssuedTokens.Where(it => !it.Value.FoundPairNode))
        {
          var containedTokens = new List<int>();
          for (var t = issuedTokens.StartToken; t < issuedTokens.RightBorder; t++)
          {
            if (nodesToSeenTokens[node].Contains(t))
            {
              containedTokens.Add(t);
            }
          }

          nodesToIssuedTokens[issuedNode].AddTokensGroup(containedTokens);

          var isPairedNode = containedTokens.Count == issuedTokens.RightBorder - issuedTokens.StartToken;
          if (!isPairedNode) continue;

          myNodePairs[issuedNode] = node;
          for (var t = issuedTokens.StartToken; t < issuedTokens.RightBorder; t++)
          {
            nodesToSeenTokens[node].Remove(t);
          }

          nodesToIssuedTokens[issuedNode].FoundPairNode = true;
        }
      }

      if (myEdges.TryGetValue(node, out var outgoingNodes))
      {
        if (outgoingNodes.Count == 1)
        {
          if (nodesToQueuedHashSets.TryGetValue(outgoingNodes[0], out var existingTokensSet))
          {
            foreach (var token in nodesToSeenTokens[node])
            {
              existingTokensSet.Add(token);
            }
          }
          else
          {
            q.Enqueue((outgoingNodes[0], tokens));
            nodesToQueuedHashSets[outgoingNodes[0]] = tokens;
          }
        }
        else
        {
          nodesToIssuedTokens[node] = new IssuedTokens(nextToken, nextToken + outgoingNodes.Count);
          foreach (var outgoingNode in outgoingNodes)
          {
            if (nodesToQueuedHashSets.TryGetValue(outgoingNode, out var existingTokensSet))
            {
              foreach (var token in nodesToSeenTokens[node])
              {
                existingTokensSet.Add(token);
              }

              existingTokensSet.Add(nextToken);
            }
            else
            {
              var newSet = new HashSet<int>(nodesToSeenTokens[node]) { nextToken };
              nodesToQueuedHashSets[outgoingNode] = newSet;
              q.Enqueue((outgoingNode, newSet));
            }

            nextToken++;
          }
        }
      }

      processed.Add(node);
    }
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

  private HorizontalCompositeBlock CreateLayout(ulong node, ulong endNode)
  {
    var layout = CreateBlockLayoutEndNodeNotInclusive(node, endNode);

    layout.InnerBlocks.Add(new SequentialBasicBlock
    {
      NodesSequence = [endNode]
    });

    return layout;
  }

  private HorizontalCompositeBlock CreateBlockLayoutEndNodeNotInclusive(ulong node, ulong endNode)
  {
    var hBlock = new HorizontalCompositeBlock();

    while (node != endNode)
    {
      if (myNodePairs.TryGetValue(node, out var pairNode))
      {
        hBlock.InnerBlocks.Add(new SequentialBasicBlock
        {
          NodesSequence = [node]
        });

        var block = new VerticalCompositeBlock();

        var outgoingNodes = myEdges[node];
        foreach (var (index, outgoingNode) in outgoingNodes.Index())
        {
          if (outgoingNode == pairNode)
          {
            block.InnerBlocks.Add(new EdgeBlock());
          }
          else
          {
            var outgoingNodeEndNode = myNodePairs.TryGetValue(outgoingNode, out var outgoingNodePair) switch
            {
              true => outgoingNodePair,
              false => pairNode
            };

            block.InnerBlocks.Add(CreateBlockLayoutEndNodeNotInclusive(outgoingNode, outgoingNodeEndNode));
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
               !myNodePairs.ContainsKey(currentNode) &&
               myEdges.ContainsKey(currentNode))
        {
          nodesSequence.Add(currentNode);
          currentNode = myEdges[currentNode].First();
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