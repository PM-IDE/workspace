using Ficus;

namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

internal static class FlamegraphContextInitializer
{
  public static void Execute(GrpcGraph graph, FlamegraphContextData data)
  {
    foreach (var edge in graph.Edges)
    {
      AddEdge(data.Edges, edge.FromNode, edge.ToNode);
      AddEdge(data.ReversedEdges, edge.ToNode, edge.FromNode);
    }

    foreach (var node in graph.Nodes)
    {
      data.IdsToNodes[node.Id] = node;
    }

    var startNode = graph.Nodes.FirstOrDefault(n => !data.ReversedEdges.ContainsKey(n.Id));
    if (startNode is null) throw new Exception("Graph does not contain a start node");

    var endNode = graph.Nodes.FirstOrDefault(n => !data.Edges.ContainsKey(n.Id));
    if (endNode is null) throw new Exception("Graph does not contain an end node");

    data.StartNode = startNode.Id;
    data.EndNode = endNode.Id;
  }

  private static void AddEdge(Dictionary<ulong, List<ulong>> map, ulong from, ulong to)
  {
    if (!map.TryGetValue(from, out var toNodes))
    {
      toNodes = [];
      map[from] = toNodes;
    }

    toNodes.Add(to);
  }
}