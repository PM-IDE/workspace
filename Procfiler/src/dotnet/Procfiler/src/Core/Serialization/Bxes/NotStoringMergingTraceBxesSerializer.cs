using System.Collections.Immutable;
using Bxes.Writer;
using Bxes.Writer.Stream;
using Procfiler.Core.Collector;
using Procfiler.Core.Serialization.Core;
using Procfiler.Utils;

namespace Procfiler.Core.Serialization.Bxes;

public class BxesWriteState
{
  public required SingleFileBxesStreamWriterImpl<BxesEvent> Writer { get; init; }
}

public class NotStoringMergingTraceBxesSerializer(
  IProcfilerLogger logger,
  bool writeAllEventData
) : NotStoringMergingTraceSerializerBase<BxesWriteState>(logger, writeAllEventData)
{
  public override void WriteTrace(string path, EventSessionInfo sessionInfo)
  {
    var writer = States.GetOrCreate(path, () => new BxesWriteState
    {
      Writer = new SingleFileBxesStreamWriterImpl<BxesEvent>(path, 0)
    });

    writer.Writer.HandleEvent(new BxesTraceVariantStartEvent(1, ImmutableArray<AttributeKeyValue>.Empty));
    foreach (var (_, @event) in new OrderedEventsEnumerator(sessionInfo.Events))
    {
      writer.Writer.HandleEvent(new BxesEventEvent<BxesEvent>(new BxesEvent(@event, WriteAllEventData)));
    }
  }

  public override void Dispose()
  {
    SerializersUtil.DisposeWriters(States.Select(pair => (pair.Key, pair.Value.Writer)), Logger, _ =>
    {
    });
  }
}