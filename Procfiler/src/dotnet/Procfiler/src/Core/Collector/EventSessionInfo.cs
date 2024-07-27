using Core.Utils;
using Procfiler.Core.EventRecord.EventsCollection;

namespace Procfiler.Core.Collector;

public record EventSessionInfo(IEnumerable<IEventsCollection> Events, SessionGlobalData GlobalData);

public class SessionGlobalData(IShadowStacks shadowStacks, long qpcSyncTime, long qpcFreq, DateTime utcSyncTime)
{
  private readonly Dictionary<long, string> myMethodIdToFqn = new();
  private readonly Dictionary<long, string> myTypeIdsToNames = new();


  public long QpcSyncTime { get; } = qpcSyncTime;
  public long QpcFreq { get; } = qpcFreq;
  public DateTime UtcSyncTime { get; } = utcSyncTime;

  public IReadOnlyDictionary<long, string> TypeIdToNames => myTypeIdsToNames;
  public IReadOnlyDictionary<long, string> MethodIdToFqn => myMethodIdToFqn;
  public IShadowStacks Stacks { get; } = shadowStacks;


  public void AddInfoFrom(EventWithGlobalDataUpdate update)
  {
    AddTypeIdWithName(update.TypeIdToName);
    AddMethodIdWithName(update.MethodIdToFqn);

    if (Stacks is IFromEventsShadowStacks fromEventsShadowStacks)
    {
      fromEventsShadowStacks.AddStack(update.OriginalEvent);
    }
  }

  private void AddMethodIdWithName(MethodIdToFqn? updateMethodIdToFqn)
  {
    if (updateMethodIdToFqn is { })
    {
      myMethodIdToFqn[updateMethodIdToFqn.Value.Id] = updateMethodIdToFqn.Value.Fqn;
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
    myMethodIdToFqn.MergeOrThrow(other.myMethodIdToFqn);
  }
}