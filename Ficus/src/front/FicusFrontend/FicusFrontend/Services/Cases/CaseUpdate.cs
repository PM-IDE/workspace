namespace FicusFrontend.Services.Cases;

public class Case
{
  public required string Name { get; init; }
  public required DateTime CreatedAt { get; init; }


  public override int GetHashCode() => Name.GetHashCode();
  public override bool Equals(object? obj) => obj is Case { Name: var name } && name == Name;
}

public abstract class CaseUpdate;

public sealed class CasesListUpdate : CaseUpdate
{
  public required Case Case { get; init; }
}

public sealed class CaseContextValuesUpdate : CaseUpdate
{
  public required string CaseName { get; init; }
  public required Guid PipelinePartGuid { get; init; }
}