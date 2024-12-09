using System.IO.Pipelines;
using Ficus;
using JetBrains.Collections.Viewable;

namespace FicusFrontend.Services.Cases;

public class Subscription
{
  public required Guid Id { get; init; }
  public required string Name { get; init; }

  public required IViewableMap<Guid, Pipeline> Pipelines { get; init; }
}

public class Pipeline
{
  public required Guid Id { get; init; }
  public required string Name { get; init; }
  public required Subscription ParentSubscription { get; init; }

  public required IViewableMap<string, ProcessData> Processes { get; init; }
}

public class ProcessData
{
  public required Pipeline ParentPipeline { get; init; }
  public required string ProcessName { get; init; }

  public required IViewableMap<string, Case> ProcessCases { get; init; }
}

public class Case
{
  public required ProcessData ParentProcess { get; init; }
  public required string Name { get; init; }
  public required DateTime CreatedAt { get; init; }
  public required IViewableMap<Guid, PipelinePartExecutionResult> ContextValues { get; init; }



  public override int GetHashCode()
  {
    return Name.GetHashCode();
  }

  public override bool Equals(object? obj)
  {
    return obj is Case { Name: var name } && name == Name;
  }
}

public class PipelinePartExecutionResult
{
  public required string PipelinePartName { get; init; }
  public required List<GrpcContextValueWithKeyName> ContextValues { get; init; }
}