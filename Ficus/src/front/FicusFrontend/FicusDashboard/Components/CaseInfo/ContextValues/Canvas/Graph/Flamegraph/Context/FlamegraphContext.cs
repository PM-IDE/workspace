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

  public HorizontalCompositeBlock Layout { get; }
  public IReadOnlyDictionary<ulong, GrpcGraphNode> IdsToNodes { get; }


  public FlamegraphContext(GrpcGraph graph)
  {
    var data = new FlamegraphContextData();

    FlamegraphContextInitializer.Execute(graph, data);
    new NodePairsFinder().Find(data);

    Layout = new FlamegraphLayoutCreator().Create(data);

    IdsToNodes = data.IdsToNodes;
  }
}