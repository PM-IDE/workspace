using Ficus;

namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

internal class FlamegraphContextData
{
  public Dictionary<ulong, GrpcGraphNode> IdsToNodes { get; } = [];
  public Dictionary<ulong, List<ulong>> Edges { get; } = [];
  public Dictionary<ulong, List<ulong>> ReversedEdges { get; } = [];
  public Dictionary<ulong, ulong> NodePairs { get; } = [];

  public ulong StartNode { get; set; }
  public ulong EndNode { get; set; }
}