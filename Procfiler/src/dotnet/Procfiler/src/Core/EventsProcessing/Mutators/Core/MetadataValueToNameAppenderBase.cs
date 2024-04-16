using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord;
using Procfiler.Utils;

namespace Procfiler.Core.EventsProcessing.Mutators.Core;

public abstract class MetadataValueToNameAppenderBase(
  IProcfilerLogger logger, bool removeProperties = false) : SingleEventMutatorBase(logger), ISingleEventMutator
{
  protected abstract IEnumerable<MetadataKeysWithTransform> Transformations { get; }


  public override IEnumerable<EventLogMutation> Mutations =>
    Transformations.Select(t => new AttributeToNameAppendMutation(EventType, t.EventClassKind, t.MetadataKey, removeProperties));


  protected override void ProcessInternal(EventRecordWithMetadata eventRecord, SessionGlobalData context)
  {
    MutatorsUtil.DevastateMetadataValuesAndAppendToName(Logger, eventRecord, Transformations, removeProperties);
  }
}