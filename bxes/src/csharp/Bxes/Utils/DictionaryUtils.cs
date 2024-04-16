namespace Bxes.Utils;

public static class DictionaryUtils
{
  public static bool DeepEquals<TKey, TValue>(this IDictionary<TKey, TValue> self, IDictionary<TKey, TValue> other)
  {
    foreach (var key in self.Keys)
    {
      if (!other.ContainsKey(key)) return false;
    }

    foreach (var key in other.Keys)
    {
      if (!self.ContainsKey(key)) return false;
    }

    foreach (var (key, value) in self)
    {
      if (!value.Equals(other[key])) return false;
    }

    return true;
  }
}