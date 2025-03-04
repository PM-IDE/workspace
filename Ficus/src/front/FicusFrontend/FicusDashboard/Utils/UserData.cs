namespace FicusDashboard.Utils;

public class Key<T>(string name)
{
  public string Name => name;
}

public abstract class UserDataHolderBase
{
  private readonly Lock mySyncObject = new();
  private readonly Dictionary<object, object> myValues = new();


  protected bool TryGetData<T>(Key<T> key, out T value)
  {
    value = default;
    lock (mySyncObject)
    {
      if (!myValues.TryGetValue(key, out var obj)) return false;

      value = (T)obj;
      return true;
    }
  }

  protected void PutData<T>(Key<T> key, T value) where T : notnull
  {
    lock (mySyncObject)
    {
      myValues[key] = value;
    }
  }
}

public sealed class UserDataHolder : UserDataHolderBase
{
  public bool TryGetData<T>(Key<T> key, out T value) => base.TryGetData(key, out value);
  public void PutData<T>(Key<T> key, T value) where T : notnull => base.PutData(key, value);
}

public static class ExtensionsForUserData
{
  public static T GetOrCreate<T>(this UserDataHolder holder, Key<T> key, Func<T> valueFactory) where T : notnull
  {
    if (holder.TryGetData(key, out var existingValue))
    {
      return existingValue;
    }

    var createdValue = valueFactory();
    holder.PutData(key, createdValue);
    return createdValue;
  }

  public static T GetOrThrow<T>(this UserDataHolder holder, Key<T> key)
  {
    if (holder.TryGetData(key, out var value)) return value;

    throw new KeyNotFoundException(key.Name);
  }
}