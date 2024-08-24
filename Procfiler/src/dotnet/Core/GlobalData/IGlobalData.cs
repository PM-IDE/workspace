namespace Core.GlobalData;

public interface IGlobalData
{
  long QpcSyncTime { get; }
  long QpcFreq { get; }
  DateTime UtcSyncTime { get; }


  string? FindTypeName(long typeId);
  string? FindMethodName(long methodId);
}