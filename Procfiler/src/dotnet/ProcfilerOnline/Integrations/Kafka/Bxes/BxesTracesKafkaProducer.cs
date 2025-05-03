using Bxes.Kafka;
using Bxes.Models.Domain.Values;
using Bxes.Writer;
using Bxes.Writer.Stream;
using Confluent.Kafka;
using Core.Bxes;
using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;
using FicusKafkaConstants;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using ProcfilerOnline.Core.Settings;

namespace ProcfilerOnline.Integrations.Kafka.Bxes;

public interface IBxesMethodsKafkaProducer : IKafkaProducer<Guid, BxesKafkaTrace>;

public class BxesKafkaCaseName
{
  public required string DisplayName { get; init; }
  public required List<string> NameParts { get; init; }
}

public class BxesKafkaTrace
{
  public required Guid CaseId { get; init; }
  public required string ProcessName { get; init; }
  public required BxesKafkaCaseName CaseName { get; init; }
  public required List<EventRecordWithMetadata> Trace { get; init; }

  public List<AttributeKeyValue> Metadata { get; } = [];
}

[AppComponent]
public class BxesTracesKafkaProducer(IOptions<OnlineProcfilerSettings> settings, IProcfilerLogger logger) : IBxesMethodsKafkaProducer
{
  private const char CaseNamePartsSeparator = ';';


  private readonly IBxesStreamWriter myWriter = new BxesKafkaStreamWriter<BxesEvent>(
    BxesUtil.CreateSystemMetadata(),
    settings.Value.KafkaSettings.TopicName,
    new ProducerConfig
    {
      BootstrapServers = settings.Value.KafkaSettings.BootstrapServers,
      MessageMaxBytes = 1_000_000_000
    });


  public void Produce(Guid key, BxesKafkaTrace trace)
  {
    List<AttributeKeyValue> metadata =
    [
      new(new BxesStringValue(FicusKafkaKeys.CaseDisplayNameKey), new BxesStringValue(trace.CaseName.DisplayName)),
      new(new BxesStringValue(FicusKafkaKeys.CaseNameParts),
      new BxesStringValue(string.Join(CaseNamePartsSeparator, trace.CaseName.NameParts))),
      new(new BxesStringValue(FicusKafkaKeys.ProcessNameKey), new BxesStringValue(trace.ProcessName)),
      new(new BxesStringValue(FicusKafkaKeys.CaseId), new BxesGuidValue(trace.CaseId))
    ];

    metadata.AddRange(trace.Metadata);

    try
    {
      myWriter.HandleEvent(new BxesTraceVariantStartEvent(1, metadata));

      foreach (var eventRecord in trace.Trace)
      {
        myWriter.HandleEvent(new BxesEventEvent<BxesEvent>(new BxesEvent(eventRecord, true)));
      }

      myWriter.HandleEvent(BxesKafkaTraceVariantEndEvent.Instance);
    }
    catch (Exception ex)
    {
      logger.LogError(
        ex,
        "Failed to produce bXES trace to kafka, process name: {ProcessName}, case name: {CaseName}, events count: {EventsCount}",
        trace.ProcessName,
        trace.CaseName,
        trace.Trace.Count
      );
    }
  }
}