namespace Bxes.Models.Domain;

public interface IEventLog : IEquatable<IEventLog>
{
  uint Version { get; }

  IEventLogMetadata Metadata { get; }
  IList<ITraceVariant> Traces { get; }
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