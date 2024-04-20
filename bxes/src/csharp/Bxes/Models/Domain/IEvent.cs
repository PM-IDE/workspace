using Bxes.Models.Domain.Values;
using Bxes.Writer;

namespace Bxes.Models.Domain;

public interface IEvent : IEquatable<IEvent>
{
  long Timestamp { get; }
  string Name { get; }

  IList<AttributeKeyValue> Attributes { get; }

  IEnumerable<BxesValue> EnumerateValues()
  {
    yield return new BxesStringValue(Name);

    foreach (var (key, value) in Attributes)
    {
      yield return key;
      yield return value;
    }
  }

  IEnumerable<AttributeKeyValue> EnumerateKeyValuePairs() => Attributes;
}

public static class EventUtil
{
  public static bool Equals(IEvent first, IEvent second)
  {
    return first.Timestamp == second.Timestamp &&
           first.Name == second.Name &&
           EventLogUtil.EqualsRegardingOrder(first.Attributes, second.Attributes);
  }
}

public class InMemoryEventImpl(
  long timestamp,
  BxesStringValue name,
  List<AttributeKeyValue> attributes
) : IEvent
{
  public long Timestamp { get; } = timestamp;
  public string Name => name.Value;
  public IList<AttributeKeyValue> Attributes { get; } = attributes;


  public bool Equals(IEvent? other) => other is InMemoryEventImpl && EventUtil.Equals(this, other);
}