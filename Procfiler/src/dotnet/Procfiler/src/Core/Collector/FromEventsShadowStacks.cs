using Procfiler.Utils;

namespace Procfiler.Core.Collector;

public interface IFromEventsShadowStacks : IShadowStacks
{
  IReadOnlyDictionary<int, StackTraceInfo> StackTraceInfos { get; }


  void AddStack(TraceEvent traceEvent);
}

public class FromEventsShadowStacks(MutableTraceEventStackSource source) : IFromEventsShadowStacks
{
  private readonly StackTracesStorage myStorage = new(source);


  public IReadOnlyDictionary<int, StackTraceInfo> StackTraceInfos => myStorage.StackTraces;


  public void AddStack(TraceEvent traceEvent)
  {
    myStorage.AddStackTraceInfoFrom(traceEvent);
  }
}

public class StackTracesStorage(MutableTraceEventStackSource source)
{
  private readonly Dictionary<int, int> myStacksHashCodesToIds = new();
  private readonly Dictionary<int, StackTraceInfo> myIdsToStackTraces = new();


  public IReadOnlyDictionary<int, StackTraceInfo> StackTraces => myIdsToStackTraces;


  public void AddStackTraceInfoFrom(TraceEvent @event)
  {
    var id = @event.CallStackIndex();
    if (id == CallStackIndex.Invalid) return;

    var intId = (int)id;
    if (myIdsToStackTraces.TryGetValue(intId, out _)) return;

    var info = @event.CreateEventStackTraceInfoOrThrow(source);
    var infoHash = info.GetHashCode();
    if (myStacksHashCodesToIds.TryGetValue(infoHash, out _))
    {
      return;
    }

    myStacksHashCodesToIds[infoHash] = intId;
    myIdsToStackTraces[intId] = info;
  }
}