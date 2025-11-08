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

public class ObjectRelations
{
  public required string Id { get; init; }
  public required List<string> RelatedObjectsIds { get; init; }
}

public class TypeObjects
{
  public required string TypeName { get; init; }
  public required List<string> ObjectIds { get; init; }
}

public class NodeObjectsState
{
  public List<TypeObjects>? InitialState { get; }
  public List<TypeObjects>? FinalState { get; }
  public List<ObjectRelations> InitialStateObjectsRelations { get; }

  public NodeObjectsState(GrpcModelElementOcelAnnotation annotation)
  {
    InitialState = CreateTypeObjectsState(annotation.InitialState);
    FinalState = CreateTypeObjectsState(annotation.FinalState);

    InitialStateObjectsRelations = annotation.Relations.Select(r => new ObjectRelations
    {
      Id = r.ObjectId,
      RelatedObjectsIds = r.RelatedObjectsIds.ToList()
    }).ToList();

    return;

    List<TypeObjects>? CreateTypeObjectsState(GrpcOcelState? state) =>
      state?.TypeStates?
        .Select(ts => new TypeObjects
        {
          TypeName = ts.Type,
          ObjectIds = ts.ObjectIds.ToList()
        })
        .ToList();
  }
}

public class EnhancedNodeDto
{
  public required EnhancedNode Node { get; init; }
  public required NodeObjectsState? Objects { get; init; }
}

public class FlamegraphSankeyObjectPart
{
  public required double From { get; init; }
  public required double To { get; init; }
}

public abstract class FlamegraphSankeyPosition
{
  public required string Id { get; init; }
}

public class FlamegraphSankeyMultiplePositions : FlamegraphSankeyPosition
{
  public List<FlamegraphSankeyObjectPart> Parts { get; } = [];
}

public class FlamegraphSankeySinglePositions : FlamegraphSankeyPosition
{
  public required FlamegraphSankeyObjectPart ObjectPart { get; init; }
}

public class FlamegraphSankeyConnection
{
  public required FlamegraphSankeyPosition From { get; init; }
  public required FlamegraphSankeyPosition To { get; init; }
}

public class FlamegraphRenderingContext
{
  private const string ObjectIdBasePart = "ocel-object-id";

  private static int ourNextContextId;


  public required FlamegraphContext Context { get; init; }
  public required Dictionary<ulong, EnhancedEdge> EnhancedEdges { get; init; }
  public required Dictionary<ulong, EnhancedNodeDto> EnhancedNodes { get; init; }
  public required bool EventClassesAsName { get; init; }
  public required bool LeftToRight { get; init; }
  public required bool RenderOcelAnnotation { get; init; }
  public required bool RenderOcelTypeNames { get; init; }

  public string MainDim => LeftToRight ? "height" : "width";
  public string SecondDim => LeftToRight ? "width" : "height";

  public string MinMainDim => "min-" + MainDim;
  public string MinSecondDim => "min-" + SecondDim;

  public string MaxMainDim => "max-" + MainDim;
  public string MaxSecondDim => "max-" + SecondDim;

  public string SecondFlexDirection => LeftToRight ? "column" : "row";
  public string MainFlexDirection => LeftToRight ? "row" : "column";

  public string WritingMode => LeftToRight ? "writing-mode: vertical-rl;" : string.Empty;

  public int ContextId { get; } = Interlocked.Increment(ref ourNextContextId);
  public string ObjectIdStart { get; }


  public FlamegraphRenderingContext()
  {
    ObjectIdStart = $"{ObjectIdBasePart}-{ContextId}";
  }


  public string GenerateOcelObjectsContainerId(string objectId, bool initialState) =>
    $"{ObjectIdStart}-{GetObjectSite(initialState)}-{objectId}";

  private static string GetObjectSite(bool initialState) => initialState switch
  {
    true => "initial",
    false => "final"
  };

  public EnhancedEdge GetEnhancedEdge(ulong fromNode, ulong toNode)
  {
    var edge = Context.NodePairsToIds[(fromNode, toNode)];
    return EnhancedEdges[edge.Id];
  }

  public EnhancedNode GetEnhancedNode(ulong node) => EnhancedNodes[node].Node;
  public NodeObjectsState? GetNodeObjectsState(ulong node) => EnhancedNodes[node].Objects;

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