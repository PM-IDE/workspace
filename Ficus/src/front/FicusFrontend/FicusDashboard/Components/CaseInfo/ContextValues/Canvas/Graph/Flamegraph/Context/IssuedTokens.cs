namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

internal class IssuedTokens(int startToken, int rightBorder)
{
  private int myNextIndex;

  private readonly List<ulong> mySyncNodes = [];
  private readonly int?[] myMergedTokens = new int?[rightBorder - startToken];


  public bool FoundPairNode { get; set; }
  public int StartToken { get; } = startToken;
  public int RightBorder { get; } = rightBorder;


  public void AddTokensGroup(List<int> tokens, ulong syncNode)
  {
    if (tokens.Count < 2 || tokens.Count == RightBorder - StartToken) return;

    foreach (var index in tokens.Select(token => token - StartToken))
    {
      if (myMergedTokens[index] is { })
      {
        throw new Exception("Some of the tokens already merged, can not merge twice");
      }

      myMergedTokens[index] = myNextIndex;
    }

    mySyncNodes.Add(syncNode);
    myNextIndex++;
  }

  public List<FlamegraphPath> GroupOutgoingNodesByPaths(List<ulong> outgoingNodes)
  {
    if (myNextIndex is 0)
    {
      return outgoingNodes.Select(n =>
      {
        var path = new FlamegraphPath();
        path.PathNodes.Add(n);
        return path;
      }).ToList();
    }

    var result = new List<FlamegraphPath>();
    var groupsLists = Enumerable.Range(0, myNextIndex).Zip(mySyncNodes).Select(n => new FlamegraphPath(n.Second)).ToArray();
    foreach (var (mergedToken, node) in myMergedTokens.Zip(outgoingNodes))
    {
      if (mergedToken is not { } token)
      {
        var path = new FlamegraphPath();
        path.PathNodes.Add(node);

        result.Add(path);

        continue;
      }

      groupsLists[token].PathNodes.Add(node);
    }

    result.AddRange(groupsLists);

    return result;
  }
}