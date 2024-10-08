using Bxes.Utils;
using Bxes.Writer;
using Bxes.Writer.Stream;

namespace Bxes.Models.Domain;

public interface IEventLogMetadata : IEquatable<IEventLogMetadata>
{
  IList<BxesExtension> Extensions { get; }
  IList<BxesClassifier> Classifiers { get; }
  IList<AttributeKeyValue> Properties { get; }
  IList<BxesGlobal> Globals { get; }

  IEnumerable<BxesStreamEvent> ToEventsStream()
  {
    foreach (var extension in Extensions)
      yield return new BxesLogMetadataExtensionEvent(extension);

    foreach (var classifier in Classifiers)
      yield return new BxesLogMetadataClassifierEvent(classifier);

    foreach (var global in Globals)
      yield return new BxesLogMetadataGlobalEvent(global);

    foreach (var property in Properties)
      yield return new BxesLogMetadataPropertyEvent(property);
  }
}

public class EventLogMetadata : IEventLogMetadata
{
  public IList<BxesExtension> Extensions { get; } = new List<BxesExtension>();
  public IList<BxesClassifier> Classifiers { get; } = new List<BxesClassifier>();
  public IList<AttributeKeyValue> Properties { get; } = new List<AttributeKeyValue>();
  public IList<BxesGlobal> Globals { get; } = new List<BxesGlobal>();


  public bool Equals(IEventLogMetadata? other)
  {
    if (ReferenceEquals(other, this)) return true;
    if (other is not EventLogMetadata) return false;

    if (other.Extensions.Count != Extensions.Count ||
        other.Classifiers.Count != Classifiers.Count ||
        other.Properties.Count != Properties.Count ||
        other.Globals.Count != Globals.Count)
    {
      return false;
    }

    if (!EventLogUtil.EqualsRegardingOrder(Extensions, other.Extensions)) return false;
    if (!EventLogUtil.EqualsRegardingOrder(Classifiers, other.Classifiers)) return false;
    if (!EventLogUtil.EqualsRegardingOrder(Properties, other.Properties)) return false;

    return EventLogUtil.EqualsRegardingOrder(Globals, other.Globals);
  }

  public override bool Equals(object? obj) => obj is EventLogMetadata other && Equals(other);

  public override int GetHashCode() =>
    HashCode.Combine(
      Extensions.CalculateHashCode(),
      Classifiers.CalculateHashCode(),
      Properties.CalculateHashCode(),
      Globals.CalculateHashCode()
    );
}