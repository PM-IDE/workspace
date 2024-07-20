namespace Core.Utils;

public static class DictionaryExtensions
{
  public static void AddOrIncrement<TKey>(this IDictionary<TKey, int> map, TKey key) where TKey : notnull
  {
    if (map.TryGetValue(key, out var count))
    {
      map[key] = count + 1;
    }
    else
    {
      map[key] = 1;
    }
  }

  public static TValue GetOrCreate<TKey, TValue>(
    this IDictionary<TKey, TValue> map, TKey key, Func<TValue> factory) where TKey : notnull
  {
    if (map.TryGetValue(key, out var existingValue))
    {
      return existingValue;
    }

    var value = factory();
    map[key] = value;
    return value;
  }

  public static TValue? GetValueOrDefault<TKey, TValue>(this IDictionary<TKey, TValue> map, TKey key)
    where TKey : notnull
  {
    if (map.TryGetValue(key, out var value)) return value;

    return default;
  }

  public static void MergeOrThrow<TKey, TValue>(
    this IDictionary<TKey, TValue> map, IDictionary<TKey, TValue> secondMap) where TKey : notnull
  {
    foreach (var (key, value) in secondMap)
    {
      if (map.TryGetValue(key, out var existingValue) && existingValue is { })
      {
        if (!existingValue.Equals(value))
        {
          throw new Exception();
        }

        continue;
      }

      map[key] = value;
    }
  }

  public static Dictionary<string, string> Copy(this Dictionary<string, string> map)
  {
    return map.ToDictionary(pair => pair.Key, pair => pair.Value);
  }
}