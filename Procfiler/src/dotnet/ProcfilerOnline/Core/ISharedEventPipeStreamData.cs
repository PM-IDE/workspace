using Core.Events.EventRecord;
using Core.GlobalData;
using Microsoft.Diagnostics.Tracing.Parsers.FrameworkEventSource;

namespace ProcfilerOnline.Core;

public interface ISharedEventPipeStreamData : IGlobalData
{
  void UpdateMethodsInfo(ExtendedMethodIdToFqn methodIdToFqn);
  void UpdateTypeIdsToNames(long typeId, string typeName);
  void UpdateSyncTimes(long qpcSyncTime, long qpcFreq, DateTime utcSyncTime);
  void UpdateManagedToNativeThread(long managedThreadId, long nativeThreadId);

  ExtendedMethodInfo? FindMethodDetails(long methodId);
  long? FindNativeThreadId(long managedThreadId);
}

public class SharedEventPipeStreamData : ISharedEventPipeStreamData
{
  private readonly Dictionary<long, ExtendedMethodInfo> myMethodIdsToFqns = [];
  private readonly Dictionary<long, string> myTypeIdsToNames = [];
  private readonly Dictionary<long, long> myManagedThreadsToNative = [];


  public long QpcSyncTime { get; private set; }
  public long QpcFreq { get; private set; }
  public DateTime UtcSyncTime { get; private set; }


  public void UpdateSyncTimes(long qpcSyncTime, long qpcFreq, DateTime utcSyncTime)
  {
    QpcFreq = qpcFreq;
    QpcSyncTime = qpcSyncTime;
    UtcSyncTime = utcSyncTime;
  }

  public void UpdateManagedToNativeThread(long managedThreadId, long nativeThreadId)
  {
    myManagedThreadsToNative[managedThreadId] = nativeThreadId;
  }

  public string? FindTypeName(long typeId) => myTypeIdsToNames.GetValueOrDefault(typeId);
  public string? FindMethodName(long methodId) => myMethodIdsToFqns.GetValueOrDefault(methodId)?.Fqn;
  public ExtendedMethodInfo? FindMethodDetails(long methodId) => myMethodIdsToFqns.GetValueOrDefault(methodId);

  public long? FindNativeThreadId(long managedThreadId) =>
    myManagedThreadsToNative.TryGetValue(managedThreadId, out var nativeThreadId) switch
    {
      true => nativeThreadId,
      false => null
    };

  public void UpdateTypeIdsToNames(long typeId, string typeName) => myTypeIdsToNames[typeId] = typeName;

  public void UpdateMethodsInfo(ExtendedMethodIdToFqn methodIdToFqn)
  {
    myMethodIdsToFqns[methodIdToFqn.Id] = methodIdToFqn.ExtendedMethodInfo;
  }
}