using Bxes.Writer;

namespace Bxes.Models;

public interface ITraceVariant : IEquatable<ITraceVariant>
{
  uint Count { get; }

  IList<AttributeKeyValue> Metadata { get; }
  IList<IEvent> Events { get; }

  IEnumerable<BxesValue> EnumerateValues()
  {
    foreach (var pair in Metadata)
    {
      yield return pair.Key;
      yield return pair.Value;
    }

    foreach (var @event in Events)
    {
      foreach (var value in @event.EnumerateValues())
      {
        yield return value;
      }
    }
  }

  IEnumerable<AttributeKeyValue> EnumerateKeyValuePairs()
  {
    foreach (var pair in Metadata)
    {
      yield return pair;
    }

    foreach (var @event in Events)
    {
      foreach (var pair in @event.EnumerateKeyValuePairs())
      {
        yield return pair;
      }
    }
  }
}

public class TraceVariantImpl(
  uint count,
  List<IEvent> events,
  List<AttributeKeyValue> metadata
) : ITraceVariant
{
  public uint Count { get; } = count;
  public IList<IEvent> Events { get; } = events;
  public IList<AttributeKeyValue> Metadata { get; } = metadata;

  public bool Equals(ITraceVariant? other)
  {
    return other is { } &&
           Count == other.Count &&
           Events.Count == other.Events.Count &&
           Events.Zip(other.Events).All(pair => pair.First.Equals(pair.Second));
  }
}