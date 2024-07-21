using Core.Collector;
using Core.Utils;
using Microsoft.Diagnostics.Tracing;

namespace ProcfilerOnline.Core;

public class OnlineEventsProcessorImpl(IProcfilerLogger logger)
{
  private readonly Dictionary<ulong, string> myIdsToMethodsFqns = new();
  private readonly Dictionary<int, ThreadEventsProcessor> myThreadsProcessors = new();


  public void Process(Stream eventPipeStream)
  {
    var source = new EventPipeEventSource(eventPipeStream);
    source.Clr.MethodLoadVerbose += traceEvent =>
    {
      myIdsToMethodsFqns[(ulong)traceEvent.MethodID] = traceEvent.MethodName;
    };

    source.Dynamic.All += traceEvent =>
    {
      if (traceEvent.ProviderName != EventPipeProvidersNames.ProcfilerCppProvider) return;

      myThreadsProcessors
        .GetOrCreate(traceEvent.ThreadID, () => new ThreadEventsProcessor(logger, traceEvent.ThreadID))
        .Process(traceEvent);
    };

    source.Process();
  }
}