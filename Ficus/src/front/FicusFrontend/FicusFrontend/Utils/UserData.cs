namespace FicusFrontend.Utils;

public class Key<T>(string name)
{
  public string Name => name;
}

public interface IUserDataHolder
{
  bool TryGetData<T>(Key<T> key, out T value);
  void PutData<T>(Key<T> key, T value);
}

public abstract class UserDataHolderBase : IUserDataHolder
{
  private readonly Lock mySyncObject = new();
  private readonly Dictionary<object, object> myValues = new();


  public virtual bool TryGetData<T>(Key<T> key, out T value)
  {
    value = default;
    lock (mySyncObject)
    {
      if (!myValues.TryGetValue(key, out var obj)) return false;

      value = (T)obj;
      return true;
    }
  }

  public virtual void PutData<T>(Key<T> key, T value) where T : notnull
  {
    lock (mySyncObject)
    {
      myValues[key] = value;
    }
  }
}

public sealed class UserDataHolder : UserDataHolderBase;

public static class ExtensionsForUserData
{
  public static T GetOrCreate<T>(this IUserDataHolder holder, Key<T> key, Func<T> valueFactory)
  {
    if (holder.TryGetData(key, out var existingValue))
    {
      return existingValue;
    }

    var createdValue = valueFactory();
    holder.PutData(key, createdValue);
    return createdValue;
  }

  public static T GetOrThrow<T>(this IUserDataHolder holder, Key<T> key)
  {
    if (holder.TryGetData(key, out var value)) return value;

    throw new KeyNotFoundException(key.Name);
  }
}