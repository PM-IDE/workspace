using Core.Events.EventRecord;
using Core.GlobalData;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.Core;

public abstract class AttributeRenamingMutatorBase(IProcfilerLogger logger, string initialName, string finalName)
  : SingleEventMutatorBase(logger)
{
  public override IEnumerable<EventLogMutation> Mutations =>
    [new AttributeRenameMutation(EventType, initialName, finalName)];


  protected override void ProcessInternal(EventRecordWithMetadata eventRecord, IGlobalData context)
  {
    eventRecord.Metadata[finalName] = eventRecord.Metadata[initialName];
    eventRecord.Metadata.Remove(initialName);
  }
}