namespace Bxes.Utils;

public static class ListExtensions
{
  public static int CalculateHashCode<T>(this IList<T> list)
  {
    const int Seed = 487;
    const int Modifier = 31;

    unchecked
    {
      return list.Aggregate(Seed, (current, item) => current * Modifier + item.GetHashCode());
    }
  }

  public static void AddRange<T>(this IList<T> list, IEnumerable<T> values)
  {
    foreach (var value in values)
    {
      list.Add(value);
    }
  }
}