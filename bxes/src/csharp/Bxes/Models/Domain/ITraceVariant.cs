using Bxes.Writer;

namespace Bxes.Models.Domain;

public interface ITraceVariant : IEquatable<ITraceVariant>
{
  uint Count { get; }

  IList<AttributeKeyValue> Metadata { get; }
  IList<IEvent> Events { get; }
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