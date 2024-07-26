using Core.Container;
using Core.Events.EventRecord;
using Core.Events.EventsCollection;
using Core.Utils;

namespace Core.Methods;

public interface IAsyncMethodsGrouper
{
  string AsyncMethodsPrefix { get; }


  IDictionary<string, List<List<EventRecordWithMetadata>>> GroupAsyncMethods(
    IDictionary<long, IEventsCollection> managedThreadsEvents);
}

[AppComponent]
public class AsyncMethodsGrouper : IAsyncMethodsGrouper
{
  public string AsyncMethodsPrefix => "ASYNC_";


  public IDictionary<string, List<List<EventRecordWithMetadata>>> GroupAsyncMethods(
    IDictionary<long, IEventsCollection> managedThreadsEvents)
  {
    var result = new Dictionary<string, List<List<EventRecordWithMetadata>>>();
    var onlineGrouper = new OnlineAsyncMethodsGrouper(AsyncMethodsPrefix, (method, traces) =>
    {
      result.GetOrCreate(method, static () => []).AddRange(traces);
    });

    foreach (var (_, events) in managedThreadsEvents)
    {
      foreach (var @event in events)
      {
        onlineGrouper.ProcessEvent(@event.Event);
      }
    }

    return result;
  }
}