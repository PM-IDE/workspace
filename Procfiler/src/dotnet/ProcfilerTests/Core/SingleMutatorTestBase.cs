using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord.EventRecord;
using Procfiler.Core.EventsProcessing.Mutators.Core;

namespace ProcfilerTests.Core;

public abstract class SingleMutatorTestBase
{
  protected abstract string EventClass { get; }


  protected abstract ISingleEventMutator CreateMutator();

  protected EventRecordWithMetadata CreateRandomEvent(EventMetadata metadata) => TestUtil.CreateRandomEvent(EventClass, metadata);

  protected void ExecuteWithRandomEvent(EventMetadata metadata, Action<EventRecordWithMetadata> action)
  {
    var eventRecord = CreateRandomEvent(metadata);
    CreateMutator().Process(eventRecord, new SessionGlobalData(EmptyShadowStacks.Instance, 0, 1, DateTime.UtcNow));

    action(eventRecord);
  }
}