using Ficus;

namespace FicusFrontend.Services.Cases;

public class Case
{
  public required string Name { get; init; }
  public required DateTime CreatedAt { get; init; }


  public override int GetHashCode() => Name.GetHashCode();
  public override bool Equals(object? obj) => obj is Case { Name: var name } && name == Name;
}

public abstract class ProcessUpdate;

public sealed class ProcessesListUpdate : ProcessUpdate
{
  public required List<ProcessData> Processes { get; init; }
}

public sealed class ProcessContextValuesUpdate : ProcessUpdate
{
  public required string CaseName { get; init; }
  public required string ProcessName { get; init; }
}