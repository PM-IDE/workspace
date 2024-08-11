using Core.Container;
using Core.Utils;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core.Updaters;

[AppComponent]
public class SyncTimesUpdater : ISharedDataUpdater
{
  public void Process(EventProcessingContext context)
  {
    if (context.TraceEvent is not { } traceEvent) return;

    var source = traceEvent.Source;
    var syncTime = source.GetSyncQpc();
    var qpcFreq = source.GetQpcFreq();
    var utcSyncDate = source.GetSyncTimeUtc();

    context.SharedData.UpdateSyncTimes(syncTime, qpcFreq, utcSyncDate);
  }
}