using Core.Constants.TraceEvents;
using Core.Container;
using Core.Events.EventRecord;
using Core.EventsProcessing.Mutators;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.GlobalData;
using Core.Utils;

namespace ProcfilerOnline.Core.Mutators;

public interface IMethodBeginEndSingleMutator : ISingleEventMutator;

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class MethodBeginEndEventMutator : IMethodBeginEndSingleMutator
{
  private readonly Dictionary<string, string> myBeginFullNamesCache = new();
  private readonly Dictionary<string, string> myEndFullNamesCache = new();

  public IEnumerable<EventLogMutation> Mutations => EmptyCollections<EventLogMutation>.EmptyList;


  public void Process(EventRecordWithMetadata eventRecord, IGlobalData context)
  {
    if (eventRecord.TryGetMethodDetails() is not var (_, methodId)) return;

    var details = context.FindMethodDetails(methodId);
    var fqn = details?.Fqn ?? TraceEventsConstants.Undefined;

    var fullNameFactory = () =>
      eventRecord.EventClass + "_{" + MutatorsUtil.TransformMethodLikeNameForEventNameConcatenation(fqn) + "}";

    var newName = eventRecord.GetMethodEventKind() switch
    {
      MethodKind.Begin => myBeginFullNamesCache.GetOrCreate(fqn, fullNameFactory),
      MethodKind.End => myEndFullNamesCache.GetOrCreate(fqn, fullNameFactory),
      _ => throw new ArgumentOutOfRangeException()
    };

    eventRecord.Metadata[TraceEventsConstants.MethodName] = details?.Name ?? TraceEventsConstants.Undefined;
    eventRecord.Metadata[TraceEventsConstants.MethodNamespace] = details?.Namespace ?? TraceEventsConstants.Undefined;
    eventRecord.Metadata[TraceEventsConstants.MethodSignature] = details?.Signature ?? TraceEventsConstants.Undefined;

    eventRecord.EventName = newName;
  }
}