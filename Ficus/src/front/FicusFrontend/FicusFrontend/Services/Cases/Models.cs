using Ficus;
using JetBrains.Collections.Viewable;

namespace FicusFrontend.Services.Cases;

public class ProcessData
{
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