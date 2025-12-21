namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

internal class NodePairsFinder
{
  private readonly HashSet<ulong> myProcessedNodes = [];
  private readonly Queue<(ulong Node, HashSet<int> Tokens)> myQueue = [];
  private readonly Dictionary<ulong, IssuedTokens> myNodesToIssuedTokens = [];
  private readonly Dictionary<ulong, HashSet<int>> myNodesToQueuedHashSets = [];

  private int myNextToken;


  public void Find(FlamegraphContextData data)
  {
    myQueue.Enqueue((data.StartNode, []));

    while (myQueue.Count > 0)
    {
      if (myQueue.All(pair => AnyIncomingNodesUnprocessed(data, pair.Node)))
      {
        throw new Exception("Queue contains only waiting for processing nodes, will livelock");
      }

      var (node, tokens) = myQueue.Dequeue();

      if (AnyIncomingNodesUnprocessed(data, node))
      {
        myQueue.Enqueue((node, tokens));
        continue;
      }

      AssertNoTokensFromPairedNodes(data, node, tokens);

      ProcessNode(data, node, tokens);
      EnqueueOutgoingNodes(data, node, tokens);
    }

    if (myProcessedNodes.Count != data.IdsToNodes.Count)
    {
      throw new Exception("There are several components in the graph, can not handle it");
    }
  }

  private void AssertNoTokensFromPairedNodes(FlamegraphContextData data, ulong node, HashSet<int> tokens)
  {
    foreach (var (issuedNode, issuedTokens) in myNodesToIssuedTokens.Where(p => p.Value.FoundPairNode))
    {
      for (var t = issuedTokens.StartToken; t < issuedTokens.RightBorder; t++)
      {
        if (!tokens.Contains(t)) continue;

        var issuedNodeName = data.IdsToNodes[issuedNode].Data;
        var currentNodeName = data.IdsToNodes[node].Data;

        throw new Exception(
          $"Node {currentNodeName} contains token {t} issued from node {issuedNodeName} when it has already found its parent");
      }
    }
  }

  private bool AnyIncomingNodesUnprocessed(FlamegraphContextData data, ulong node)
  {
    var anyIncomingNodeUnprocessed = false;
    if (data.ReversedEdges.TryGetValue(node, out var incomingNodes))
    {
      anyIncomingNodeUnprocessed = incomingNodes.Any(incomingNode => !myProcessedNodes.Contains(incomingNode));
    }

    return anyIncomingNodeUnprocessed;
  }

  private void ProcessNode(FlamegraphContextData data, ulong node, HashSet<int> tokens)
  {
    myProcessedNodes.Add(node);
    myNodesToQueuedHashSets.Remove(node);

    if (!data.ReversedEdges.TryGetValue(node, out var incomingNodes) || incomingNodes.Count < 2) return;

    foreach (var (issuedNode, issuedTokens) in myNodesToIssuedTokens.Where(it => !it.Value.FoundPairNode))
    {
      var containedTokens = new List<int>();
      for (var t = issuedTokens.StartToken; t < issuedTokens.RightBorder; t++)
      {
        if (tokens.Contains(t))
        {
          containedTokens.Add(t);
        }
      }

      issuedTokens.AddTokensGroup(containedTokens, node);

      var isPairedNode = containedTokens.Count == issuedTokens.RightBorder - issuedTokens.StartToken;
      if (!isPairedNode) continue;

      var issuedNodeOutgoingNodes = data.Edges.GetValueOrDefault(issuedNode) ?? [];
      data.NodePairs[issuedNode] = new NodePair(node, issuedTokens.GroupOutgoingNodesByPaths(issuedNodeOutgoingNodes));

      for (var t = issuedTokens.StartToken; t < issuedTokens.RightBorder; t++)
      {
        tokens.Remove(t);
      }

      issuedTokens.FoundPairNode = true;
    }
  }

  private void EnqueueOutgoingNodes(FlamegraphContextData data, ulong node, HashSet<int> tokens)
  {
    if (!data.Edges.TryGetValue(node, out var outgoingNodes)) return;

    if (outgoingNodes.Count == 1)
    {
      EnqueueOutgoingNode(outgoingNodes[0], tokens, null);
    }
    else
    {
      myNodesToIssuedTokens[node] = new IssuedTokens(myNextToken, myNextToken + outgoingNodes.Count);
      foreach (var outgoingNode in outgoingNodes)
      {
        EnqueueOutgoingNode(outgoingNode, tokens, myNextToken);
        myNextToken++;
      }
    }
  }

  private void EnqueueOutgoingNode(ulong outgoingNode, HashSet<int> tokens, int? newToken)
  {
    if (myNodesToQueuedHashSets.TryGetValue(outgoingNode, out var existingTokensSet))
    {
      foreach (var token in tokens)
      {
        existingTokensSet.Add(token);
      }

      if (newToken.HasValue)
      {
        existingTokensSet.Add(newToken.Value);
      }
    }
    else
    {
      var newSet = new HashSet<int>(tokens);

      if (newToken.HasValue)
      {
        newSet.Add(newToken.Value);
      }

      myNodesToQueuedHashSets[outgoingNode] = newSet;
      myQueue.Enqueue((outgoingNode, newSet));
    }
  }
}