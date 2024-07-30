using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Commands.CollectClrEvents.Split;
using Procfiler.Core.EventRecord;
using Procfiler.Core.EventRecord.EventsCollection;

namespace Procfiler.Core.SplitByMethod;

public class SplitterImplementation(
  IProcfilerLogger logger,
  IProcfilerEventsFactory eventsFactory,
  IEnumerable<EventRecordWithPointer> events,
  string filterPattern,
  InlineMode inlineMode
)
{
  private readonly Dictionary<string, IReadOnlyList<IReadOnlyList<EventRecordWithMetadata>>> myResult = new();


  public IReadOnlyDictionary<string, IReadOnlyList<IReadOnlyList<EventRecordWithMetadata>>> Split()
  {
    var splitter = new CallbackBasedSplitter<List<EventRecordWithMetadata>>(
      logger, events, filterPattern, inlineMode, static _ => new List<EventRecordWithMetadata>(), HandleUpdate);

    splitter.Split();
    return myResult;
  }

  private void HandleUpdate(EventUpdateBase<List<EventRecordWithMetadata>> update)
  {
    switch (update)
    {
      case MethodStartedUpdate<List<EventRecordWithMetadata>> methodStartedUpdate:
        break;
      case NormalEventUpdate<List<EventRecordWithMetadata>> normalEventUpdate:
      {
        HandleNormalUpdate(normalEventUpdate);
        return;
      }
      case MethodFinishedUpdate<List<EventRecordWithMetadata>> methodFinishedUpdate:
      {
        HandleMethodFinishedUpdate(methodFinishedUpdate);
        return;
      }
      case MethodExecutionUpdate<List<EventRecordWithMetadata>> methodExecutionUpdate:
      {
        HandleMethodExecutionUpdate(methodExecutionUpdate);
        return;
      }
      default:
        throw new ArgumentOutOfRangeException();
    }
  }

  private static void HandleNormalUpdate(NormalEventUpdate<List<EventRecordWithMetadata>> normalEventUpdate)
  {
    normalEventUpdate.FrameInfo.State!.Add(normalEventUpdate.Event.DeepClone());
  }

  private void HandleMethodFinishedUpdate(MethodFinishedUpdate<List<EventRecordWithMetadata>> methodFinishedUpdate)
  {
    var stateEvents = methodFinishedUpdate.FrameInfo.State!;

    if (stateEvents.Count <= 0) return;

    var existingValue =
      myResult.GetOrCreate(methodFinishedUpdate.FrameInfo.Frame, static () => new List<List<EventRecordWithMetadata>>());

    var listOfListOfEvents = (List<List<EventRecordWithMetadata>>)existingValue;
    listOfListOfEvents.Add(stateEvents);
  }

  private void HandleMethodExecutionUpdate(MethodExecutionUpdate<List<EventRecordWithMetadata>> methodExecutionUpdate)
  {
    var currentTopmost = methodExecutionUpdate.FrameInfo;
    var contextEvent = currentTopmost.State!.Count switch
    {
      > 0 => currentTopmost.State[^1],
      _ => null
    };

    var executionEvent = CurrentFrameInfoUtil.CreateMethodExecutionEvent(
      currentTopmost, eventsFactory, methodExecutionUpdate.MethodName, contextEvent);

    currentTopmost.State.Add(executionEvent);
  }
}