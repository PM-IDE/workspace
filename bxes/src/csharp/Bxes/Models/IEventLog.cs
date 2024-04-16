using System.Runtime;
using Bxes.Models.Values;
using Bxes.Utils;
using Bxes.Writer;
using Bxes.Writer.Stream;

namespace Bxes.Models;

public interface IEventLog : IEquatable<IEventLog>
{
  uint Version { get; }

  IEventLogMetadata Metadata { get; }
  IList<ITraceVariant> Traces { get; }
}

public interface IEventLogMetadata : IEquatable<IEventLogMetadata>
{
  IList<BxesExtension> Extensions { get; }
  IList<BxesClassifier> Classifiers { get; }
  IList<AttributeKeyValue> Properties { get; }
  IList<BxesGlobal> Globals { get; }

  IEnumerable<BxesValue> EnumerateValues()
  {
    foreach (var (key, value) in Properties)
    {
      yield return key;
      yield return value;
    }

    foreach (var extension in Extensions)
    {
      yield return extension.Name;
      yield return extension.Prefix;
      yield return extension.Uri;
    }

    foreach (var classifier in Classifiers)
    {
      yield return classifier.Name;

      foreach (var key in classifier.Keys)
      {
        yield return key;
      }
    }

    foreach (var global in Globals)
    {
      foreach (var attribute in global.Globals)
      {
        yield return attribute.Key;
        yield return attribute.Value;
      }
    }
  }

  IEnumerable<AttributeKeyValue> EnumerateKeyValuePairs()
  {
    foreach (var pair in Properties)
    {
      yield return pair;
    }

    foreach (var global in Globals)
    {
      foreach (var attribute in global.Globals)
      {
        yield return attribute;
      }
    }
  }

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

public enum GlobalsEntityKind : byte
{
  Event = 0,
  Trace = 1,
  Log = 2
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

    if (other is null ||
        other.Extensions.Count != Extensions.Count ||
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

  public override int GetHashCode()
  {
    return HashCode.Combine(
      Extensions.CalculateHashCode(),
      Classifiers.CalculateHashCode(),
      Properties.CalculateHashCode(),
      Globals.CalculateHashCode()
    );
  }
}

public record BxesClassifier
{
  public required List<BxesStringValue> Keys { get; init; }
  public required BxesStringValue Name { get; init; }


  public virtual bool Equals(BxesClassifier? other) =>
    other is { } &&
    Name.Equals(other.Name) &&
    EventLogUtil.EqualsRegardingOrder(Keys, other.Keys);

  public override int GetHashCode()
  {
    var nameHash = Name.GetHashCode();
    foreach (var key in Keys.OrderBy(key => key.Value))
    {
      nameHash = HashCode.Combine(nameHash, key.GetHashCode());
    }

    return nameHash;
  }
}

public record BxesExtension
{
  public required BxesStringValue Prefix { get; init; }
  public required BxesStringValue Uri { get; init; }
  public required BxesStringValue Name { get; init; }


  public virtual bool Equals(BxesExtension? other) =>
    other is { } &&
    Prefix.Equals(other.Prefix) &&
    Uri.Equals(other.Uri) &&
    Name.Equals(other.Name);

  public override int GetHashCode() => HashCode.Combine(Prefix, Uri, Name);
}

public record BxesGlobal
{
  public required GlobalsEntityKind Kind { get; init; }
  public required List<AttributeKeyValue> Globals { get; init; }

  public virtual bool Equals(BxesGlobal? other) =>
    other is { } &&
    other.Kind == Kind &&
    EventLogUtil.EqualsRegardingOrder(Globals, other.Globals);

  public override int GetHashCode()
  {
    var kindHash = Kind.GetHashCode();
    foreach (var kv in Globals.OrderBy(key => key.Key.Value))
    {
      kindHash = HashCode.Combine(kv.GetHashCode(), kindHash);
    }

    return kindHash;
  }
}

public class InMemoryEventLog(uint version, IEventLogMetadata metadata, List<ITraceVariant> traces) : IEventLog
{
  public uint Version { get; } = version;

  public IEventLogMetadata Metadata { get; } = metadata;
  public IList<ITraceVariant> Traces { get; } = traces;


  public bool Equals(IEventLog? other)
  {
    return other is { } &&
           Version == other.Version &&
           Metadata.Equals(other.Metadata) &&
           Traces.Count == other.Traces.Count &&
           Traces.Zip(other.Traces).All(pair => pair.First.Equals(pair.Second));
  }
}

public static class EventLogUtil
{
  public static IEnumerable<BxesStreamEvent> ToEventsStream(this IEventLog log)
  {
    foreach (var @event in log.Metadata.ToEventsStream())
    {
      yield return @event;
    }

    foreach (var variant in log.Traces)
    {
      yield return new BxesTraceVariantStartEvent(variant.Count, variant.Metadata);

      foreach (var @event in variant.Events)
      {
        yield return new BxesEventEvent<IEvent>(@event);
      }
    }
  }

  public static bool Equals(ICollection<AttributeKeyValue> first, ICollection<AttributeKeyValue> second)
  {
    return first.Count == second.Count &&
           first.Zip(second).All(pair =>
             pair.First.Key.Equals(pair.Second.Key) && pair.First.Value.Equals(pair.Second.Value));
  }

  public static bool EqualsRegardingOrder<T>(IList<T> firstList, IList<T> secondList)
  {
    if (firstList.Count != secondList.Count) return false;

    var firstSet = firstList.ToHashSet();
    var secondSet = secondList.ToHashSet();

    foreach (var first in firstSet)
    {
      if (!secondSet.Contains(first)) return false;
    }

    foreach (var second in secondSet)
    {
      if (!firstSet.Contains(second)) return false;
    }

    return true;
  }
}