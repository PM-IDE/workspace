using Core.Container;
using Core.Utils;
using ProcfilerOnline.Core.Processors;

namespace ProcfilerOnline.Core.Updaters;

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