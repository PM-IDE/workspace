﻿using System.Diagnostics;
using Core.Constants.TraceEvents;
using Core.Container;
using Core.Events.EventRecord;
using Core.EventsProcessing.Mutators.Core;
using Core.EventsProcessing.Mutators.Core.Passes;
using Core.GlobalData;
using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.ArrayPools;

public abstract class BufferEventTypeNameMutator(IProcfilerLogger logger) : SingleEventMutatorBase(logger)
{
  private const string Buffer = TraceEventsConstants.BufferEventType;


  public override IEnumerable<EventLogMutation> Mutations =>
    [new EventTypeNameMutation(EventType, $"{Buffer}/{EventType[Buffer.Length..]}")];

  protected override void ProcessInternal(EventRecordWithMetadata eventRecord, IGlobalData context)
  {
    Debug.Assert(eventRecord.EventClass.StartsWith(Buffer));
    Debug.Assert(eventRecord.EventName.StartsWith(Buffer));

    const string EventTypeSeparator = TraceEventsConstants.EventTypeSeparator;
    eventRecord.EventName = eventRecord.EventName.Insert(Buffer.Length, EventTypeSeparator);
    eventRecord.EventClass = eventRecord.EventClass.Insert(Buffer.Length, EventTypeSeparator);
  }
}

//todo: seems very bad, need to refactor
[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class BufferAllocatedEventTypeMutator(IProcfilerLogger logger) : BufferEventTypeNameMutator(logger)
{
  public override string EventType => TraceEventsConstants.BufferAllocated;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class BufferRentedEventTypeMutator(IProcfilerLogger logger) : BufferEventTypeNameMutator(logger)
{
  public override string EventType => TraceEventsConstants.BufferRented;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class BufferReturnedEventTypeMutator(IProcfilerLogger logger) : BufferEventTypeNameMutator(logger)
{
  public override string EventType => TraceEventsConstants.BufferReturned;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class BufferTrimmedEventTypeMutator(IProcfilerLogger logger) : BufferEventTypeNameMutator(logger)
{
  public override string EventType => TraceEventsConstants.BufferTrimmed;
}

[EventMutator(SingleEventMutatorsPasses.SingleEventsMutators)]
public class BufferTrimPollTypeMutator(IProcfilerLogger logger) : BufferEventTypeNameMutator(logger)
{
  public override string EventType => TraceEventsConstants.BufferTrimPoll;
}