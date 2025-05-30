﻿using Core.Events.EventRecord;
using Core.Utils;
using Procfiler.Core.EventRecord.EventsCollection;

namespace Procfiler.Core.Collector;

public record EventSessionInfo(IEnumerable<IEventsCollection> Events, SessionGlobalData GlobalData);

public class SessionGlobalData(IShadowStacks shadowStacks, long qpcSyncTime, long qpcFreq, DateTime utcSyncTime) : IGlobalDataWithStacks
{
  private readonly Dictionary<long, ExtendedMethodInfo> myMethodIdToMethodInfo = new();
  private readonly Dictionary<long, string> myTypeIdsToNames = new();


  public long QpcSyncTime { get; } = qpcSyncTime;
  public long QpcFreq { get; } = qpcFreq;
  public DateTime UtcSyncTime { get; } = utcSyncTime;
  public IShadowStacks Stacks { get; } = shadowStacks;


  public string? FindTypeName(long typeId) => myTypeIdsToNames.GetValueOrDefault(typeId);
  public ExtendedMethodInfo? FindMethodDetails(long methodId) => myMethodIdToMethodInfo.GetValueOrDefault(methodId);

  public void AddInfoFrom(EventWithGlobalDataUpdate update)
  {
    AddTypeIdWithName(update.TypeIdToName);
    AddMethodIdWithName(update.MethodIdToFqn);

    if (Stacks is IFromEventsShadowStacks fromEventsShadowStacks)
    {
      fromEventsShadowStacks.AddStack(update.OriginalEvent);
    }
  }

  private void AddMethodIdWithName(MethodIdToMethodInfo? updateMethodIdToFqn)
  {
    if (updateMethodIdToFqn is { })
    {
      myMethodIdToMethodInfo[updateMethodIdToFqn.Value.Id] = updateMethodIdToFqn.Value.Info;
    }
  }

  private void AddTypeIdWithName(TypeIdToName? recordTypeIdToName)
  {
    if (recordTypeIdToName.HasValue)
    {
      myTypeIdsToNames[recordTypeIdToName.Value.Id] = recordTypeIdToName.Value.Name;
    }
  }

  public void MergeWith(SessionGlobalData other)
  {
    myTypeIdsToNames.MergeOrThrow(other.myTypeIdsToNames);
    myMethodIdToMethodInfo.MergeOrThrow(other.myMethodIdToMethodInfo);
  }
}