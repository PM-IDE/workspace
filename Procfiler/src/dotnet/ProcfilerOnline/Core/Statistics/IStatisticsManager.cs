using Core.Events.EventRecord;
using Core.Utils;

namespace ProcfilerOnline.Core.Statistics;

public interface IStatisticsManager
{
  void Log(IProcfilerLogger logger);

  void UpdateProcessedEventStatistics(EventRecordWithMetadata eventRecord);
}