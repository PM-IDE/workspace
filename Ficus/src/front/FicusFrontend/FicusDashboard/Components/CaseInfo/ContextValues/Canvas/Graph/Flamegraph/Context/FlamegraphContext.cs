using Ficus;

namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

public class FlamegraphContext
{
  public static FlamegraphContext? TryCreate(GrpcGraph graph)
  {
    try
    {
      return new FlamegraphContext(graph);
    }
    catch
    {
      return null;
    }
  }

  public GrpcGraph Graph { get; }
  public HorizontalCompositeBlock Layout { get; }
  public IReadOnlyDictionary<ulong, GrpcGraphNode> IdsToNodes { get; }
  public IReadOnlyDictionary<(ulong, ulong), GrpcGraphEdge> NodePairsToIds { get; }

  private FlamegraphContext(GrpcGraph graph)
  {
    Graph = graph;
    var data = new FlamegraphContextData();

    FlamegraphContextInitializer.Execute(graph, data);
    new NodePairsFinder().Find(data);

    Layout = new FlamegraphLayoutCreator().Create(data);

    IdsToNodes = data.IdsToNodes;
    NodePairsToIds = data.NodePairsToEdges;
  }
}

public class FlamegraphRenderingContext
{
  public required FlamegraphContext Context { get; init; }
  public required Dictionary<ulong, EnhancedEdge> EnhancedEdges { get; init; }
  public required bool EventClassesAsName { get; init; }


  public EnhancedEdge GetEnhancedEdge(ulong fromNode, ulong toNode)
  {
    var edge = Context.NodePairsToIds[(fromNode, toNode)];
    return EnhancedEdges[edge.Id];
  }

  public string GetNodeName(ulong nodeId) => EventClassesAsName switch
  {
    true => CreateEventClassesName(nodeId),
    false => Context.IdsToNodes[nodeId].Data
  };

  private string CreateEventClassesName(ulong nodeId) =>
    string.Join(
      "\n",
      Context.IdsToNodes[nodeId].AdditionalData
        .Where(x => x.SoftwareData?.Histogram is { })
        .SelectMany(x => x.SoftwareData.Histogram.Select(h => h.Name))
        .Where(x => x is { })
        .Distinct()
        .Order()
        .Take(3)
    );
}