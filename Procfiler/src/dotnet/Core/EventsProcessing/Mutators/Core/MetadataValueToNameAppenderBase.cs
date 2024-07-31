using Core.Events.EventRecord;
using Core.GlobalData;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.Core;

public abstract class MetadataValueToNameAppenderBase(
  IProcfilerLogger logger,
  bool removeProperties = false) : SingleEventMutatorBase(logger), ISingleEventMutator
{
  protected abstract IEnumerable<MetadataKeysWithTransform> Transformations { get; }


  public override IEnumerable<EventLogMutation> Mutations =>
    Transformations.Select(t => new AttributeToNameAppendMutation(EventType, t.EventClassKind, t.MetadataKey, removeProperties));


  protected override void ProcessInternal(EventRecordWithMetadata eventRecord, IGlobalData context)
  {
    MutatorsUtil.DevastateMetadataValuesAndAppendToName(Logger, eventRecord, Transformations, removeProperties);
  }
}