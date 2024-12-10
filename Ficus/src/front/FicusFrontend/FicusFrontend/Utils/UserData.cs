namespace FicusFrontend.Utils;

public record Key<T>(string Name);

public interface IUserDateHolder
{
  bool Contains<T>(Key<T> key);
  T? TryGetData<T>(Key<T> key);
  void PutData<T>(Key<T> key, T value);
}

public class UserDateHolderBase : IUserDateHolder
{
  private readonly object mySyncObject = new();
  private readonly Dictionary<object, object> myValues = new();


  public bool Contains<T>(Key<T> key)
  {
    lock (mySyncObject)
    {
      return myValues.ContainsKey(key);
    }
  }

  public T? TryGetData<T>(Key<T> key)
  {
    lock (mySyncObject)
    {
      if (myValues.TryGetValue(key, out var value))
      {
        return (T)value;
      }

      return default;
    }
  }

  public void PutData<T>(Key<T> key, T value) where T : notnull
  {
    lock (mySyncObject)
    {
      myValues[key] = value;
    }
  }
}

public static class ExtensionsForUserData
{
  public static T GetOrCreate<T>(this IUserDateHolder holder, Key<T> key, Func<T> valueFactory)
  {
    if (holder.TryGetData(key) is { } existingValue)
    {
      return existingValue;
    }

    var createdValue = valueFactory();
    holder.PutData(key, createdValue);
    return createdValue;
  }
}