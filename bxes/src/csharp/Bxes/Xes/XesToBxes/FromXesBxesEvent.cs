using Bxes.Models;
using Bxes.Writer;

namespace Bxes.Xes.XesToBxes;

public readonly struct FromXesBxesEvent : IEvent
{
  public required long Timestamp { get; init; }
  public required string Name { get; init; }
  public required IList<AttributeKeyValue> Attributes { get; init; }


  public bool Equals(IEvent? other) => other is { } && EventUtil.Equals(this, other);
}