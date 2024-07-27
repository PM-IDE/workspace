using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord.EventRecord;

namespace Procfiler.Core.EventsProcessing.Mutators.Core;

public abstract class MetadataValuesRemover(IProcfilerLogger logger) : SingleEventMutatorBase(logger), ISingleEventMutator
{
  protected abstract string[] MetadataKeys { get; }


  public override IEnumerable<EventLogMutation> Mutations => MetadataKeys.Select(key => new AttributeRemovalMutation(EventType, key));


  protected override void ProcessInternal(EventRecordWithMetadata eventRecord, SessionGlobalData context)
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