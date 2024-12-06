using System.IO.Pipelines;
using Ficus;
using JetBrains.Collections.Viewable;

namespace FicusFrontend.Services.Cases;

public class Subscription
{
  public required Guid Id { get; init; }
  public required string Name { get; init; }

  public required Dictionary<Guid, Pipeline> Pipelines { get; init; }
}

public class Pipeline
{
  public required Guid Id { get; init; }
  public required string Name { get; init; }
  public required Subscription ParentSubscription { get; init; }

  public required Dictionary<string, ProcessData> Processes { get; init; }
}

public class ProcessData
{
  public required Pipeline ParentPipeline { get; init; }
  public required string ProcessName { get; init; }

  public required Dictionary<string, CaseData> ProcessCases { get; init; }
}

public class CaseData
{
  public required Case Case { get; init; }
  public required ViewableMap<Guid, PipelinePartExecutionResult> ContextValues { get; init; }

  public class PipelinePartExecutionResult
  {
    public required string PipelinePartName { get; init; }
    public required List<GrpcContextValueWithKeyName> ContextValues { get; init; }
  }
}