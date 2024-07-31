using Core.Container;
using Core.Events.EventRecord;
using Core.EventsProcessing.Mutators;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.GlobalData;
using Core.Utils;

namespace ProcfilerOnline.Core.Mutators;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class MethodBeginEndEventMutator : ISingleEventMutator
{
  public IEnumerable<EventLogMutation> Mutations => EmptyCollections<EventLogMutation>.EmptyList;


  public void Process(EventRecordWithMetadata eventRecord, IGlobalData context)
  {
    if (eventRecord.TryGetMethodDetails() is not var (_, methodId)) return;

    var fqn = context.MethodIdToFqn.GetValueOrDefault(methodId) ?? "UNRESOLVED";
    var newName = eventRecord.EventClass + "_{" + MutatorsUtil.TransformMethodLikeNameForEventNameConcatenation(fqn) + "}";
    eventRecord.EventName = newName;
  }
}