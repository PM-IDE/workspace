using Bxes.Models.Domain.Values;
using Bxes.Writer;

namespace Bxes.Models.Domain;

public interface IEvent : IEquatable<IEvent>
{
  long Timestamp { get; }
  string Name { get; }

  IList<AttributeKeyValue> Attributes { get; }
}

public static class EventUtil
{
  public static bool Equals(IEvent first, IEvent second) =>
    first.Timestamp == second.Timestamp &&
    first.Name == second.Name &&
    EventLogUtil.EqualsRegardingOrder(first.Attributes, second.Attributes);
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