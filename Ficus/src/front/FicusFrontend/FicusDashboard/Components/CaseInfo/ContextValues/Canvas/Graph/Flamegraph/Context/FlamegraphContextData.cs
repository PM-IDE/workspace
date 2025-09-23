using Ficus;

namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

internal class FlamegraphPath(ulong? syncNode = null)
{
  public List<ulong> PathNodes { get; } = [];
  public ulong? SyncNode { get; } = syncNode;
}

internal class NodePair(ulong pairedNode, List<FlamegraphPath> paths)
{
  internal List<FlamegraphPath> Paths { get; } = paths;
  internal ulong PairedNode { get; } = pairedNode;
}

internal class FlamegraphContextData
{
  public Dictionary<ulong, GrpcGraphNode> IdsToNodes { get; } = [];
  public Dictionary<ulong, List<ulong>> Edges { get; } = [];
  public Dictionary<ulong, List<ulong>> ReversedEdges { get; } = [];
  public Dictionary<ulong, NodePair> NodePairs { get; } = [];

  public ulong StartNode { get; set; }
  public ulong EndNode { get; set; }
}