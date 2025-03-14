using Ficus;
using FicusDashboard.Utils;
using JetBrains.Collections.Viewable;

namespace FicusDashboard.Services.Cases;

public class Subscription : FrontModelBase
{
  public required Guid Id { get; init; }
  public required string Name { get; init; }

  public required IViewableMap<Guid, Pipeline> Pipelines { get; init; }
}

public class Pipeline : FrontModelBase
{
  public required Guid Id { get; init; }
  public required string Name { get; init; }
  public required Subscription ParentSubscription { get; init; }

  public required IViewableMap<string, ProcessData> Processes { get; init; }
}

public class ProcessData : FrontModelBase
{
  public required Pipeline ParentPipeline { get; init; }
  public required string ProcessName { get; init; }

  public required IViewableMap<string, Case> ProcessCases { get; init; }
}

public class Case : FrontModelBase
{
  public required ProcessData ParentProcess { get; init; }
  public required List<string> NameParts { get; init; }
  public required string FullName { get; init; }
  public required string DisplayName { get; init; }
  public required DateTime CreatedAt { get; init; }
  public required IViewableMap<Guid, List<PipelinePartExecutionResult>> ExecutionResults { get; init; }


  public override int GetHashCode() => FullName.GetHashCode();

  public override bool Equals(object? obj) => obj is Case { FullName: var fullName } && fullName == FullName;
}

public class PipelinePartExecutionResult : FrontModelBase
{
  public required string PipelinePartName { get; init; }
  public required List<ContextValueWrapper> ContextValues { get; init; }
}

public class ContextValueWrapper(GrpcContextValueWithKeyName value)
{
  public Guid Id { get; } = Guid.NewGuid();
  public GrpcContextValueWithKeyName Value { get; } = value;
}