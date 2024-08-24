using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;

namespace ProcfilerOnline.Core.Statistics;

[AppComponent]
public class StatisticsManager : IStatisticsManager
{
  private readonly ProcessedEventsStatistics myEventsStatistics = new();


  public void Log(IProcfilerLogger logger)
  {
    myEventsStatistics.Log(logger);
  }

  public void UpdateProcessedEventStatistics(EventRecordWithMetadata eventRecord)
  {
    myEventsStatistics.UpdateProcessedEventsStatistics(eventRecord);
  }
}