namespace FicusFrontend.Services;

public class Case
{
  public required string Name { get; init; }
}

public interface ICasesService
{
  IAsyncEnumerable<Case> OpenCasesStream();
}

public class CasesService : ICasesService
{
  private readonly List<Case> myCases =
  [
    new() { Name = "Case 1" },
    new() { Name = "Case 2" },
    new() { Name = "Case 3" },
    new() { Name = "Case 4" }
  ];


  public async IAsyncEnumerable<Case> OpenCasesStream()
  {
    foreach (var @case in myCases)
    {
      yield return @case;
      await Task.Delay(1000);
    }
  }
}