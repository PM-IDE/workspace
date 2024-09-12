namespace FicusFrontend.Services;

public class Case
{
  public required string Name { get; init; }
  public required DateTime CreatedAt { get; init; }
}

public interface ICasesService
{
  IAsyncEnumerable<Case> OpenCasesStream();
}

public class CasesService : ICasesService
{
  private readonly List<Case> myCases =
  [
    new() { Name = "Case 1", CreatedAt = DateTime.Now },
    new() { Name = "Case 2", CreatedAt = DateTime.Now },
    new() { Name = "Case 3", CreatedAt = DateTime.Now },
    new() { Name = "Case 4", CreatedAt = DateTime.Now }
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