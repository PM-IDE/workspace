namespace Bxes.Utils;

public static class ListExtensions
{
  extension<T>(IList<T> list)
  {
    public int CalculateHashCode()
    {
      const int Seed = 487;
      const int Modifier = 31;

      unchecked
      {
        return list.Aggregate(Seed, (current, item) => current * Modifier + item.GetHashCode());
      }
    }

    public void AddRange(IEnumerable<T> values)
    {
      foreach (var value in values)
      {
        list.Add(value);
      }
    }
  }
}