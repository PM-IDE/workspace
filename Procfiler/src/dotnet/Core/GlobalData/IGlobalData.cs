namespace Core.GlobalData;

public interface IGlobalData
{
  long QpcSyncTime { get; }
  long QpcFreq { get; }
  DateTime UtcSyncTime { get; }

  IReadOnlyDictionary<long, string> TypeIdToNames { get; }
  IReadOnlyDictionary<long, string> MethodIdToFqn { get; }
}