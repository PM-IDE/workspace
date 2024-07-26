using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord;

namespace Procfiler.Core.EventsProcessing.Mutators.Core;

public abstract class AttributeRenamingMutatorBase(IProcfilerLogger logger, string initialName, string finalName)
  : SingleEventMutatorBase(logger)
{
  public override IEnumerable<EventLogMutation> Mutations =>
    new[] { new AttributeRenameMutation(EventType, initialName, finalName) };


  protected override void ProcessInternal(EventRecordWithMetadata eventRecord, SessionGlobalData context)
  {
    eventRecord.Metadata[finalName] = eventRecord.Metadata[initialName];
    eventRecord.Metadata.Remove(initialName);
  }
}