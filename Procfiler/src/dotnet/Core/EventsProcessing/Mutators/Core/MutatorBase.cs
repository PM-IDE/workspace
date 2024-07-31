using Core.Events.EventRecord;
using Core.GlobalData;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.Core;

public abstract class MutatorBase(IProcfilerLogger logger)
{
  protected readonly IProcfilerLogger Logger = logger;
}

public abstract class SingleEventMutatorBase(IProcfilerLogger logger) : MutatorBase(logger), ISingleEventMutator
{
  public abstract string EventType { get; }
  public abstract IEnumerable<EventLogMutation> Mutations { get; }


  public void Process(EventRecordWithMetadata eventRecord, IGlobalData context)
  {
    if (eventRecord.EventClass == EventType)
    {
      ProcessInternal(eventRecord, context);
    }
  }

  protected abstract void ProcessInternal(EventRecordWithMetadata eventRecord, IGlobalData context);
}