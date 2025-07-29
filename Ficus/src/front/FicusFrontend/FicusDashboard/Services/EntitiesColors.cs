namespace FicusDashboard.Services;

public interface IEntitiesColors
{
  string GetOrCreateColor(string entity);
}

public class EntitiesColors : IEntitiesColors
{
  private readonly HashSet<string> myUsedColors = [];
  private readonly Dictionary<string, string> myColors = [];


  public string GetOrCreateColor(string entity) => NextColor(entity);


  private string NextColor(string key)
  {
    if (myColors.TryGetValue(key, out var color))
    {
      return color;
    }

    color = GenerateRandomColor();

    while (myUsedColors.Contains(color))
    {
      color = GenerateRandomColor();
    }

    myColors[key] = color;
    myUsedColors.Add(color);

    return color;
  }

  private static string GenerateRandomColor() => $"#{Random.Shared.Next(0x1000000):X6}";
}