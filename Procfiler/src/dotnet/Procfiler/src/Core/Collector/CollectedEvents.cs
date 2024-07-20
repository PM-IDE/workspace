using Core.Utils;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventsCollection;
using Procfiler.Utils;

namespace Procfiler.Core.Collector;

public readonly record struct CollectedEvents(
  IEventsCollection Events,
  SessionGlobalData GlobalData
);

public readonly record struct TypeIdToName(long Id, string Name);

public readonly record struct MethodIdToFqn(long Id, string Fqn);

public readonly record struct EventWithGlobalDataUpdate(
  TraceEvent OriginalEvent,
  EventRecordWithMetadata Event,
  TypeIdToName? TypeIdToName,
  MethodIdToFqn? MethodIdToFqn
);

public readonly record struct CreatingEventContext(MutableTraceEventStackSource Source, TraceLog Log);

public record StackTraceInfo(int StackTraceId, int ManagedThreadId, string[] Frames)
{
  protected virtual bool PrintMembers(StringBuilder builder)
  {
    builder
      .LogPrimitiveValue(nameof(StackTraceId), StackTraceId)
      .Append(StringBuilderExtensions.SerializeValue(Frames));

    return true;
  }

  public override int GetHashCode()
  {
    if (Frames.Length == 0) return ManagedThreadId;

    var hash = Frames[0].AsSpan().CalculateHash();

    for (var i = 1; i < Frames.Length; ++i)
    {
      hash = HashCode.Combine(hash, Frames[i].AsSpan().CalculateHash());
    }

    return HashCode.Combine(hash, ManagedThreadId);
  }
}