using System.Collections;
using System.Diagnostics;
using Core.CommandLine;
using Microsoft.Diagnostics.Tracing;

namespace Core.Events.EventRecord;

public class TooMuchMetadataValuesException : ProcfilerException;

//For better times, TraceEvent.Clone() consumes too much memory
public class EventMetadataBasedOnTraceEvent(TraceEvent traceEvent) : IEventMetadata
{
  private enum KeyOrValue
  {
    Key,
    Value
  }


  private EventMetadata? myInsertedMetadata;
  private int myDeletedProperties;
  private int myTraceEventMetadataCount = traceEvent.PayloadNames.Length;

  public TraceEvent TraceEvent { get; } = traceEvent;
  public bool IsReadOnly => false;
  public int MaxMetadataValuesCount => sizeof(int) * 8;
  public int Count => myTraceEventMetadataCount + (myInsertedMetadata?.Count ?? 0);
  public ICollection<string> Keys => CreateArrayOfNotDeletedElements(KeyOrValue.Key);
  public ICollection<string> Values => CreateArrayOfNotDeletedElements(KeyOrValue.Value);


  private string[] CreateArrayOfNotDeletedElements(KeyOrValue keyOrValue)
  {
    var result = new string[Count];
    var names = TraceEvent.PayloadNames;
    var index = 0;

    for (var i = 0; i < names.Length; i++)
    {
      if (IsDeletedAt(i)) continue;

      result[index++] = keyOrValue switch
      {
        KeyOrValue.Key => TraceEvent.PayloadNames[i],
        KeyOrValue.Value => TraceEvent.PayloadString(i),
        _ => throw new ArgumentOutOfRangeException(nameof(keyOrValue), keyOrValue, null)
      };
    }

    if (myInsertedMetadata is not { } insertedMetadata) return result;

    var insertedCollection = keyOrValue switch
    {
      KeyOrValue.Key => insertedMetadata.Keys,
      KeyOrValue.Value => insertedMetadata.Values,
      _ => throw new ArgumentOutOfRangeException(nameof(keyOrValue), keyOrValue, null)
    };

    foreach (var value in insertedCollection)
    {
      result[index++] = value;
    }

    return result;
  }

  private bool IsDeletedAt(int index)
  {
    AssertIndex(index);
    return (myDeletedProperties & 1 << index) == 1;
  }

  private void AssertIndex(int index)
  {
    Debug.Assert(index >= 0 && index < TraceEvent.PayloadNames.Length);
    if (index >= MaxMetadataValuesCount)
    {
      throw new TooMuchMetadataValuesException();
    }
  }

  public IEnumerator<KeyValuePair<string, string>> GetEnumerator()
  {
    var names = TraceEvent.PayloadNames;
    for (var i = 0; i < names.Length; i++)
    {
      if (!IsDeletedAt(i))
      {
        yield return new KeyValuePair<string, string>(names[i], TraceEvent.PayloadString(i));
      }
    }

    if (myInsertedMetadata is { } insertedMetadata)
    {
      foreach (var keyValuePair in insertedMetadata)
      {
        yield return keyValuePair;
      }
    }
  }

  public void Add(KeyValuePair<string, string> item)
  {
    EnsureInsertedEventsCreated();
    Debug.Assert(myInsertedMetadata is { });
    myInsertedMetadata.Add(item);
  }

  private void EnsureInsertedEventsCreated()
  {
    myInsertedMetadata ??= new EventMetadata();
  }

  public void Clear()
  {
    myInsertedMetadata?.Clear();
    myInsertedMetadata = null;
    for (var i = 0; i < TraceEvent.PayloadNames.Length; i++)
    {
      MarkAsDeleted(i);
    }

    myTraceEventMetadataCount = 0;
  }

  private void MarkAsDeleted(int index)
  {
    AssertIndex(index);
    myDeletedProperties |= 1 << index;
  }

  public bool Contains(KeyValuePair<string, string> item) => ContainsKey(item.Key);

  private bool ContainsInInsertedEvents(string key) =>
    myInsertedMetadata is { } insertedMetadata && insertedMetadata.ContainsKey(key);

  private int? TryFindIndexFor(string key)
  {
    for (var i = 0; i < TraceEvent.PayloadNames.Length; i++)
    {
      if (!IsDeletedAt(i) && TraceEvent.PayloadNames[i] == key)
      {
        return i;
      }
    }

    return null;
  }

  public void CopyTo(KeyValuePair<string, string>[] array, int arrayIndex)
  {
    foreach (var keyValuePair in this)
    {
      array[arrayIndex++] = keyValuePair;
    }

    if (myInsertedMetadata is { } insertedMetadata)
    {
      insertedMetadata.CopyTo(array, arrayIndex);
    }
  }

  public bool Remove(KeyValuePair<string, string> item) => Remove(item.Key);

  public void Add(string key, string value)
  {
    if (ContainsKey(key)) throw new ArgumentException(key);

    EnsureInsertedEventsCreated();
    Debug.Assert(myInsertedMetadata is { });
    myInsertedMetadata.Add(key, value);
  }

  public bool ContainsKey(string key) => TryFindIndexFor(key) is { } || ContainsInInsertedEvents(key);

  public bool Remove(string key)
  {
    if (TryFindIndexFor(key) is { } index)
    {
      MarkAsDeleted(index);
      myTraceEventMetadataCount--;
      return true;
    }

    if (myInsertedMetadata is { } insertedMetadata)
    {
      return insertedMetadata.Remove(key);
    }

    return false;
  }

  public bool TryGetValue(string key, out string value)
  {
    if (TryFindIndexFor(key) is { } index)
    {
      value = TraceEvent.PayloadString(index);
      return true;
    }

    if (myInsertedMetadata is { } insertedMetadata)
    {
      return insertedMetadata.TryGetValue(key, out value);
    }

    value = string.Empty;
    return false;
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
      if (TryFindIndexFor(key) is { } index)
      {
        MarkAsDeleted(index);
        EnsureInsertedEventsCreated();
        Debug.Assert(myInsertedMetadata is { });
        myInsertedMetadata.Add(key, value);
        return;
      }

      if (myInsertedMetadata is null)
      {
        EnsureInsertedEventsCreated();
        Debug.Assert(myInsertedMetadata is { });
        myInsertedMetadata.Add(key, value);
        return;
      }

      myInsertedMetadata[key] = value;
    }
  }

  IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
}