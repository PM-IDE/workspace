namespace Procfiler.Core.EventRecord.EventRecord;

public interface IEventMetadata : IDictionary<string, string>;

public class EventMetadata : IEventMetadata
{
  private readonly List<string?> myNames;
  private readonly List<string?> myValues;

  public bool IsReadOnly => false;
  public ICollection<string> Keys => CreateArrayOfNotNullElementsFrom(myNames, Count);
  public ICollection<string> Values => CreateArrayOfNotNullElementsFrom(myValues, Count);

  public int Count { get; private set; }


  public EventMetadata(TraceEvent traceEvent)
  {
    var length = traceEvent.PayloadNames.Length;
    myValues = new List<string?>(length);
    myNames = new List<string?>(length);

    for (var i = 0; i < length; i++)
    {
      var serializedValue = Convert.ToString(traceEvent.PayloadValue(i)) ??
                            traceEvent.PayloadString(i);

      myValues.Add(string.Intern(serializedValue));
      myNames.Add(string.Intern(traceEvent.PayloadNames[i]));
    }

    Count = length;
  }

  public EventMetadata(IEventMetadata otherMetadata)
  {
    myValues = [];
    myNames = [];

    foreach (var (key, value) in otherMetadata)
    {
      Add(key, value);
    }
  }

  public EventMetadata()
  {
    const int EmptyMetadataExpectedAttributesCount = 0;
    myValues = new List<string?>(EmptyMetadataExpectedAttributesCount);
    myNames = new List<string?>(EmptyMetadataExpectedAttributesCount);

    Count = EmptyMetadataExpectedAttributesCount;
  }


  public void Add(KeyValuePair<string, string> item) => Add(item.Key, item.Value);

  public void Clear()
  {
    for (var i = 0; i < myNames.Count; i++)
    {
      myNames[i] = null;
      myValues[i] = null;
    }

    Count = 0;
  }

  public bool Contains(KeyValuePair<string, string> item) => FindKey(item.Key) > -1;

  private int FindKey(string key)
  {
    for (var i = 0; i < myNames.Count; i++)
    {
      if (myNames[i] == key)
      {
        return i;
      }
    }

    return -1;
  }

  public void CopyTo(KeyValuePair<string, string>[] array, int arrayIndex)
  {
    foreach (var keyValuePair in this)
    {
      array[arrayIndex++] = keyValuePair;
    }
  }

  public bool Remove(KeyValuePair<string, string> item) => Remove(item.Key);

  public void Add(string key, string value)
  {
    if (ContainsKey(key)) throw new ArgumentException(key);

    myNames.Add(string.Intern(key));
    myValues.Add(string.Intern(value));
    ++Count;
  }

  public bool ContainsKey(string key) => FindKey(key) > -1;

  public bool Remove(string key)
  {
    var index = FindKey(key);
    if (index < 0) return false;

    myNames[index] = null;
    myValues[index] = null;
    --Count;
    return true;
  }

  public bool TryGetValue(string key, out string value)
  {
    value = string.Empty;
    var index = FindKey(key);
    if (index < 0) return false;

    var foundValue = myValues[index];
    Debug.Assert(foundValue is { });
    value = foundValue;
    return true;
  }

  public string this[string key]
  {
    get
    {
      if (!TryGetValue(key, out var value)) throw new KeyNotFoundException(key);

      return value;
    }

    set
    {
      if (FindKey(key) is var index && index < 0)
      {
        Add(key, value);
        return;
      }

      myValues[index] = value;
    }
  }

  private static string[] CreateArrayOfNotNullElementsFrom(List<string?> givenList, int resultArraySize)
  {
    var values = new string[resultArraySize];
    var index = 0;
    for (var i = 0; i < givenList.Count; i++)
    {
      if (givenList[i] is not { } value) continue;

      values[index++] = value;
    }

    return values;
  }

  public IEnumerator<KeyValuePair<string, string>> GetEnumerator()
  {
    for (var i = 0; i < myNames.Count; i++)
    {
      if (myNames[i] is not { } name) continue;

      var value = myValues[i];
      Debug.Assert(value is { });
      yield return new KeyValuePair<string, string>(name, value);
    }
  }

  IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
}