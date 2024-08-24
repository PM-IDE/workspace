using System.Text.Json;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Extensions.Logging;

namespace ProcfilerOnline.Core.Statistics;

internal class ProcessedEventsStatistics
{
  private readonly Dictionary<string, int> myEventClassesToCounts = [];


  public void UpdateProcessedEventsStatistics(EventRecordWithMetadata eventRecord)
  {
    myEventClassesToCounts.AddOrIncrement(eventRecord.EventClass);
  }

  public void Log(IProcfilerLogger logger)
  {
    var serializedStatistics = JsonSerializer.Serialize(myEventClassesToCounts);
    logger.LogInformation("Processed events statistics: {Statistics}", serializedStatistics);
  }
}