using Bxes.Kafka;
using Bxes.Models.Domain.Values;
using Bxes.Writer;
using Bxes.Writer.Stream;
using Confluent.Kafka;
using Core.Bxes;
using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using ProcfilerOnline.Core.Settings;

namespace ProcfilerOnline.Integrations.Kafka.Bxes;

public interface IBxesMethodsKafkaProducer : IKafkaProducer<Guid, BxesKafkaTrace>;

public class BxesKafkaTrace
{
  public required string ProcessName { get; init; }
  public required string CaseName { get; init; }
  public required List<EventRecordWithMetadata> Trace { get; init; }

  public List<AttributeKeyValue> Metadata { get; } = [];
}

[AppComponent]
public class BxesTracesKafkaProducer(IOptions<OnlineProcfilerSettings> settings, IProcfilerLogger logger) : IBxesMethodsKafkaProducer
{
  private readonly IBxesStreamWriter myWriter = new BxesKafkaStreamWriter<BxesEvent>(
    BxesUtil.CreateSystemMetadata(),
    settings.Value.KafkaSettings.TopicName,
    new ProducerConfig
    {
      BootstrapServers = settings.Value.KafkaSettings.BootstrapServers
    });


  public void Produce(Guid key, BxesKafkaTrace message)
  {
    List<AttributeKeyValue> metadata =
    [
      new(new BxesStringValue("case_name"), new BxesStringValue(message.CaseName)),
      new(new BxesStringValue("process_name"), new BxesStringValue(message.ProcessName))
    ];

    metadata.AddRange(message.Metadata);

    try
    {
      myWriter.HandleEvent(new BxesTraceVariantStartEvent(1, metadata));

      foreach (var eventRecord in message.Trace)
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
        message.ProcessName,
        message.CaseName,
        message.Trace.Count
      );
    }
  }
}