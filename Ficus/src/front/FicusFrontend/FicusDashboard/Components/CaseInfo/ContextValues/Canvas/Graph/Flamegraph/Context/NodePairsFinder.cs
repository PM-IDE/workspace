namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

internal class NodePairsFinder
{
  private readonly HashSet<ulong> myProcessedNodes = [];
  private readonly Queue<(ulong, HashSet<int>)> myQueue = [];
  private readonly Dictionary<ulong, IssuedTokens> myNodesToIssuedTokens = [];
  private readonly Dictionary<ulong, HashSet<int>> myNodesToQueuedHashSets = [];

  private int myNextToken;


  public void Find(FlamegraphContextData data)
  {
    myQueue.Enqueue((data.StartNode, []));

    while (myQueue.Count > 0)
    {
      var (node, tokens) = myQueue.Dequeue();

      if (AnyIncomingNodesUnprocessed(data, node))
      {
        myQueue.Enqueue((node, tokens));
        continue;
      }

      ProcessNode(data, node, tokens);
      EnqueueOutgoingNodes(data, node, tokens);
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

      issuedTokens.AddTokensGroup(containedTokens);

      var isPairedNode = containedTokens.Count == issuedTokens.RightBorder - issuedTokens.StartToken;
      if (!isPairedNode) continue;

      data.NodePairs[issuedNode] = node;
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