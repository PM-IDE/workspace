﻿using Core.Events.EventRecord;
using Core.GlobalData;

namespace ProcfilerOnline.Core;

public interface ISharedEventPipeStreamData : IGlobalData
{
  void UpdateMethodsInfo(ExtendedMethodIdToFqn methodIdToFqn);
  void UpdateTypeIdsToNames(long typeId, string typeName);
  void UpdateSyncTimes(long qpcSyncTime, long qpcFreq, DateTime utcSyncTime);
}

public class SharedEventPipeStreamData : ISharedEventPipeStreamData
{
  private readonly Dictionary<long, ExtendedMethodInfo> myMethodIdsToInfos = new();
  private readonly Dictionary<long, string> myTypeIdsToNames = new();


  public long QpcSyncTime { get; private set; }
  public long QpcFreq { get; private set; }
  public DateTime UtcSyncTime { get; private set; }


  public void UpdateSyncTimes(long qpcSyncTime, long qpcFreq, DateTime utcSyncTime)
  {
    QpcFreq = qpcFreq;
    QpcSyncTime = qpcSyncTime;
    UtcSyncTime = utcSyncTime;
  }


  public string? FindTypeName(long typeId) => myTypeIdsToNames.GetValueOrDefault(typeId);
  public string? FindMethodName(long methodId) => myMethodIdsToInfos.GetValueOrDefault(methodId)?.Fqn;

  public ExtendedMethodInfo? FindMethodDetails(long methodId)
  {
    return myMethodIdsToInfos.GetValueOrDefault(methodId);
  }

  public void UpdateTypeIdsToNames(long typeId, string typeName) => myTypeIdsToNames[typeId] = typeName;

  public void UpdateMethodsInfo(ExtendedMethodIdToFqn methodIdToFqn)
  {
    myMethodIdsToInfos[methodIdToFqn.Id] = methodIdToFqn.ExtendedMethodInfo;
  }
}