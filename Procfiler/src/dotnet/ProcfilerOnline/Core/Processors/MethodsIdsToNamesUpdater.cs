using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;

namespace ProcfilerOnline.Core.Processors;

[AppComponent]
public class MethodsIdsToNamesUpdater : ISharedDataUpdater
{
  public void Process(EventProcessingContext context)
  {
    if (context.Event.TryGetMethodInfo() is { Id: var methodId, Fqn: var fqn })
    {
      context.SharedData.UpdateMethodsInfo(methodId, fqn);
    }
  }
}

[AppComponent]
public class SyncTimesUpdater : ISharedDataUpdater
{
  public void Process(EventProcessingContext context)
  {
    var source = context.TraceEvent.Source;
    var syncTime = source.GetSyncQpc();
    var qpcFreq = source.GetQpcFreq();
    var utcSyncDate = source.GetSyncTimeUtc();

    context.SharedData.UpdateSyncTimes(syncTime, qpcFreq, utcSyncDate);
  }
}