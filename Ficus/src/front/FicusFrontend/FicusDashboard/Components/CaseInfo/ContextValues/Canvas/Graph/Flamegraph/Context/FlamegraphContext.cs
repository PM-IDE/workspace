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
  public required Dictionary<ulong, EnhancedNode> EnhancedNodes { get; init; }
  public required bool EventClassesAsName { get; init; }
  public required bool LeftToRight { get; init; }

  public string MainDim => LeftToRight ? "height" : "width";
  public string SecondDim => LeftToRight ? "width" : "height";

  public string MinMainDim => "min-" + MainDim;
  public string MinSecondDim => "min-" + SecondDim;

  public string MaxMainDim => "max-" + MainDim;
  public string MaxSecondDim => "max-" + SecondDim;

  public string SecondFlexDirection => LeftToRight ? "column" : "row";
  public string MainFlexDirection => LeftToRight ? "row" : "column";


  public EnhancedEdge GetEnhancedEdge(ulong fromNode, ulong toNode)
  {
    var edge = Context.NodePairsToIds[(fromNode, toNode)];
    return EnhancedEdges[edge.Id];
  }

  public List<string> GetNodeName(ulong nodeId) => EventClassesAsName switch
  {
    true => GetTopThreeEventClasses(nodeId),
    false => GetDefaultNodeName(nodeId)
  };

  public (string?, string?) AdjustWidthAndHeight(string? originalWidth, string? originalHeight)
  {
    return !LeftToRight ? (originalHeight, originalWidth) : (originalWidth, originalHeight);
  }

  private List<string> GetDefaultNodeName(ulong nodeId) => [Context.IdsToNodes[nodeId].Data];

  private List<string> GetTopThreeEventClasses(ulong nodeId) =>
    Context.IdsToNodes[nodeId].AdditionalData
        .Where(x => x.SoftwareData?.Histogram is { })
        .SelectMany(x => x.SoftwareData.Histogram.Select(h => h.Name))
        .Where(x => x is { })
        .Distinct()
        .Order()
        .Take(3)
        .ToList() switch
      {
        { Count: > 0 } eventClasses => eventClasses,
        _ => GetDefaultNodeName(nodeId)
      };
}