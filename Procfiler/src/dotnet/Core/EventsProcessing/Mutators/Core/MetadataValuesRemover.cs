using Core.Events.EventRecord;
using Core.GlobalData;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.Core;

public abstract class MetadataValuesRemover(IProcfilerLogger logger) : SingleEventMutatorBase(logger), ISingleEventMutator
{
  protected abstract string[] MetadataKeys { get; }


  public override IEnumerable<EventLogMutation> Mutations => MetadataKeys.Select(key => new AttributeRemovalMutation(EventType, key));


  protected override void ProcessInternal(EventRecordWithMetadata eventRecord, IGlobalData context)
  {
    var metadata = eventRecord.Metadata;
    foreach (var metadataKey in MetadataKeys)
    {
      if (!metadata.Remove(metadataKey))
      {
        Logger.LogAbsenceOfMetadata(EventType, metadataKey);
      }
    }
  }
}