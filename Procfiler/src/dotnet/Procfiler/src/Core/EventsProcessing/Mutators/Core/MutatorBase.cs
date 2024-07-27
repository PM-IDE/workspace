using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.EventRecord.EventRecord;

namespace Procfiler.Core.EventsProcessing.Mutators.Core;

public abstract class MutatorBase(IProcfilerLogger logger)
{
  protected readonly IProcfilerLogger Logger = logger;
}

public abstract class SingleEventMutatorBase(IProcfilerLogger logger) : MutatorBase(logger), ISingleEventMutator
{
  public abstract string EventType { get; }
  public abstract IEnumerable<EventLogMutation> Mutations { get; }


  public void Process(EventRecordWithMetadata eventRecord, SessionGlobalData context)
  {
    if (eventRecord.EventClass == EventType)
    {
      ProcessInternal(eventRecord, context);
    }
  }

  protected abstract void ProcessInternal(EventRecordWithMetadata eventRecord, SessionGlobalData context);
}