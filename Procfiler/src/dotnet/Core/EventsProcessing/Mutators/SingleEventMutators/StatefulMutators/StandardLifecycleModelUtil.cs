﻿using Core.Constants.XesLifecycle;
using Core.Events.EventRecord;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.StatefulMutators;

public static class StandardLifecycleModelUtil
{
  public static void MarkAsScheduled(EventRecordWithMetadata eventRecord, string activityId)
  {
    AddCommonAttributes(eventRecord, activityId);
    eventRecord.Metadata[XesStandardLifecycleConstants.Transition] = XesStandardLifecycleConstants.Schedule;
  }

  public static void AddCommonAttributes(EventRecordWithMetadata eventRecord, string activityId)
  {
    eventRecord.Metadata[XesStandardLifecycleConstants.ActivityId] = activityId;
  }

  public static void MarkAsStarted(EventRecordWithMetadata eventRecord, string activityId)
  {
    AddCommonAttributes(eventRecord, activityId);
    eventRecord.Metadata[XesStandardLifecycleConstants.Transition] = XesStandardLifecycleConstants.Start;
  }

  public static void MarkAsCompleted(EventRecordWithMetadata eventRecord, string activityId)
  {
    AddCommonAttributes(eventRecord, activityId);
    eventRecord.Metadata[XesStandardLifecycleConstants.Transition] = XesStandardLifecycleConstants.Complete;
  }

  public static void MarkAsUnknown(EventRecordWithMetadata eventRecord, string activityId)
  {
    AddCommonAttributes(eventRecord, activityId);
    eventRecord.Metadata[XesStandardLifecycleConstants.Transition] = XesStandardLifecycleConstants.Complete;
  }
}