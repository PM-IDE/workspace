namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

internal class IssuedTokens(int startToken, int rightBorder)
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