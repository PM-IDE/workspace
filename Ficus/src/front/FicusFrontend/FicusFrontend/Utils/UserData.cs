namespace FicusFrontend.Utils;

public class Key<T>(string name)
{
  public string Name => name;
}

public interface IUserDateHolder
{
  bool TryGetData<T>(Key<T> key, out T value);
  void PutData<T>(Key<T> key, T value);
}

public class UserDateHolderBase : IUserDateHolder
{
  private readonly Lock mySyncObject = new();
  private readonly Dictionary<object, object> myValues = new();


  public bool TryGetData<T>(Key<T> key, out T value)
  {
    value = default;
    lock (mySyncObject)
    {
      if (!myValues.TryGetValue(key, out var obj)) return false;

      value = (T)obj;
      return true;
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
    if (holder.TryGetData(key, out var existingValue))
    {
      return existingValue;
    }

    var createdValue = valueFactory();
    holder.PutData(key, createdValue);
    return createdValue;
  }

  public static T GetOrThrow<T>(this IUserDateHolder holder, Key<T> key)
  {
    if (holder.TryGetData(key, out var value)) return value;

    throw new KeyNotFoundException(key.Name);
  }
}