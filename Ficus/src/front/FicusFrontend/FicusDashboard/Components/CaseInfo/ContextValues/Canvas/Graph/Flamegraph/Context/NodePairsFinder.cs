namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

internal static class NodePairsFinder
{
  public static void Find(FlamegraphContextData data)
  {
    var processed = new HashSet<ulong>();
    var q = new Queue<(ulong, HashSet<int>)>();
    var nextToken = 0;
    var nodesToIssuedTokens = new Dictionary<ulong, IssuedTokens>();
    var nodesToQueuedHashSets = new Dictionary<ulong, HashSet<int>>();

    q.Enqueue((data.StartNode, []));

    while (q.Count > 0)
    {
      var (node, tokens) = q.Dequeue();
      var anyIncomingNodeUnprocessed = false;
      if (data.ReversedEdges.TryGetValue(node, out var incomingNodes))
      {
        anyIncomingNodeUnprocessed = incomingNodes.Any(incomingNode => !processed.Contains(incomingNode));
      }

      if (anyIncomingNodeUnprocessed)
      {
        q.Enqueue((node, tokens));
        continue;
      }

      nodesToQueuedHashSets.Remove(node);
      if (data.ReversedEdges.TryGetValue(node, out incomingNodes) && incomingNodes.Count > 1)
      {
        foreach (var (issuedNode, issuedTokens) in nodesToIssuedTokens.Where(it => !it.Value.FoundPairNode))
        {
          var containedTokens = new List<int>();
          for (var t = issuedTokens.StartToken; t < issuedTokens.RightBorder; t++)
          {
            if (tokens.Contains(t))
            {
              containedTokens.Add(t);
            }
          }

          nodesToIssuedTokens[issuedNode].AddTokensGroup(containedTokens);

          var isPairedNode = containedTokens.Count == issuedTokens.RightBorder - issuedTokens.StartToken;
          if (!isPairedNode) continue;

          data.NodePairs[issuedNode] = node;
          for (var t = issuedTokens.StartToken; t < issuedTokens.RightBorder; t++)
          {
            tokens.Remove(t);
          }

          nodesToIssuedTokens[issuedNode].FoundPairNode = true;
        }
      }

      if (data.Edges.TryGetValue(node, out var outgoingNodes))
      {
        if (outgoingNodes.Count == 1)
        {
          if (nodesToQueuedHashSets.TryGetValue(outgoingNodes[0], out var existingTokensSet))
          {
            foreach (var token in tokens)
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
              foreach (var token in tokens)
              {
                existingTokensSet.Add(token);
              }

              existingTokensSet.Add(nextToken);
            }
            else
            {
              var newSet = new HashSet<int>(tokens) { nextToken };
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
}