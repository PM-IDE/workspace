﻿using Autofac;
using Core.Container;
using Core.Utils;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Core.Features;
using ProcfilerOnline.Integrations.Kafka.Bxes;
using ProcfilerOnline.Integrations.Kafka.Json;

namespace ProcfilerOnline.Core.Handlers;

public class MethodExecutionEvent : IEventPipeStreamEvent
{
  public required string ApplicationName { get; init; }
  public required TargetMethodFrame Frame { get; init; }
}

[AppComponent]
public class CompletedMethodExecutionHandler(IComponentContext container, IProcfilerLogger logger) : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (!ProcfilerOnlineFeatures.ProduceEventsToKafka.IsEnabled()) return;
    if (eventPipeStreamEvent is not MethodExecutionEvent @event) return;

    if (@event.Frame.MethodInfo is null)
    {
      logger.LogWarning("Encountered an event without MethodInfo, will not send it");
      return;
    }

    if (ProcfilerOnlineFeatures.ProduceBxesKafkaEvents.IsEnabled())
    {
      ProduceBxesKafkaMessage(@event);
      return;
    }

    ProduceJsonKafkaMessage(@event);
  }

  private void ProduceBxesKafkaMessage(MethodExecutionEvent @event)
  {
    var message = new BxesKafkaTrace
    {
      ProcessName = @event.ApplicationName,
      CaseName = @event.Frame.MethodInfo!.ToBxesKafkaCaseName(),
      Trace = @event.Frame.InnerEvents,
      CaseId = @event.Frame.CaseId
    };

    @event.Frame.MethodInfo.AddToMetadata(message.Metadata);

    container.Resolve<IBxesMethodsKafkaProducer>().Produce(Guid.NewGuid(), message);
  }

  private void ProduceJsonKafkaMessage(MethodExecutionEvent @event)
  {
    var message = new JsonMethodsExecutionKafkaMessage
    {
      Events = @event.Frame.InnerEvents.Select(JsonEventRecordWithMetadataKafkaDto.FromEventRecord).ToList(),
      MethodFullName = @event.Frame.MethodInfo!.Fqn
    };

    container.Resolve<IJsonMethodsKafkaProducer>().Produce(Guid.NewGuid(), message);
  }
}