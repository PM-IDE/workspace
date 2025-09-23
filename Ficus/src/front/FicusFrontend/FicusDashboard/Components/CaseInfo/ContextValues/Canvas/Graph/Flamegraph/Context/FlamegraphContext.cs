using Ficus;

namespace FicusDashboard.Components.CaseInfo.ContextValues.Canvas.Graph.Flamegraph.Context;

public class FlamegraphContext
{
  public HorizontalCompositeBlock Layout { get; }
  public IReadOnlyDictionary<ulong, GrpcGraphNode> IdsToNodes { get; }


  public FlamegraphContext(GrpcGraph graph)
  {
    var data = new FlamegraphContextData();

    FlamegraphContextInitializer.Execute(graph, data);
    new NodePairsFinder().Find(data);

    Layout = FlamegraphLayoutCreator.Create(data);

    IdsToNodes = data.IdsToNodes;
  }
}