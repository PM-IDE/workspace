using Core.GlobalData;

namespace ProcfilerOnline.Core;

public interface ISharedEventPipeStreamData : IGlobalData
{
  void UpdateMethodsInfo(long methodId, string fqn);
  void UpdateTypeIdsToNames(long typeId, string typeName);
  void UpdateSyncTimes(long qpcSyncTime, long qpcFreq, DateTime utcSyncTime);
}

public class SharedEventPipeStreamData : ISharedEventPipeStreamData
{
  private readonly Dictionary<long, string> myMethodIdsToFqns = new();
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
  public string? FindMethodName(long methodId) => myMethodIdsToFqns.GetValueOrDefault(methodId);

  public void UpdateMethodsInfo(long methodId, string fqn) => myMethodIdsToFqns[methodId] = fqn;
  public void UpdateTypeIdsToNames(long typeId, string typeName) => myTypeIdsToNames[typeId] = typeName;
}