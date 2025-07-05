using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection;
using Procfiler.Core.Serialization.Core;

namespace Procfiler.Core.SplitByMethod;

public class SplitterImplementation(
  IProcfilerLogger logger,
  IProcfilerEventsFactory eventsFactory,
  IEnumerable<EventRecordWithPointer> events,
  string filterPattern,
  InlineMode inlineMode
) : IOnlineMethodsSerializer
{
  private readonly Dictionary<string, List<List<EventRecordWithMetadata>>> myResult = new();


  public IReadOnlyDictionary<string, List<List<EventRecordWithMetadata>>> Split()
  {
    var splitter = new CallbackBasedSplitter(logger, events, filterPattern, inlineMode, this);

    splitter.Split();
    return myResult;
  }

  public object? CreateState(EventRecordWithMetadata eventRecord) => new List<EventRecordWithMetadata>();

  public void HandleUpdate(EventUpdateBase update)
  {
    if (update.FrameInfo.State is not List<EventRecordWithMetadata> state) return;

    switch (update)
    {
      case MethodStartedUpdate:
        break;
      case NormalEventUpdate normalEventUpdate:
      {
        HandleNormalUpdate(normalEventUpdate, state);
        return;
      }
      case MethodFinishedUpdate methodFinishedUpdate:
      {
        HandleMethodFinishedUpdate(methodFinishedUpdate, state);
        return;
      }
      case MethodExecutionUpdate methodExecutionUpdate:
      {
        HandleMethodExecutionUpdate(methodExecutionUpdate, state);
        return;
      }
      default:
        throw new ArgumentOutOfRangeException();
    }
  }

  private static void HandleNormalUpdate(NormalEventUpdate normalEventUpdate, List<EventRecordWithMetadata> state)
  {
    state.Add(normalEventUpdate.Event.DeepClone());
  }

  private void HandleMethodFinishedUpdate(MethodFinishedUpdate methodFinishedUpdate, List<EventRecordWithMetadata> state)
  {
    if (state.Count <= 0) return;

    var existingValue = myResult.GetOrCreate(methodFinishedUpdate.FrameInfo.Frame, static () => []);

    existingValue.Add(state);
  }

  private void HandleMethodExecutionUpdate(MethodExecutionUpdate methodExecutionUpdate, List<EventRecordWithMetadata> state)
  {
    var contextEvent = state.Count switch
    {
      > 0 => state[^1],
      _ => null
    };

    var executionEvent = CurrentFrameInfoUtil.CreateMethodExecutionEvent(
      methodExecutionUpdate.FrameInfo, eventsFactory, methodExecutionUpdate.MethodName, contextEvent);

    state.Add(executionEvent);
  }

  public void Dispose()
  {
  }
}