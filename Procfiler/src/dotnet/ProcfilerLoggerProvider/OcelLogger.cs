using System.Runtime.CompilerServices;
using ProcfilerEventSources;

namespace ProcfilerLoggerProvider;

public abstract class OcelObjectBase
{
  private static long ourNextId;

  internal long Id { get; }


  protected OcelObjectBase()
  {
    Id = Interlocked.Increment(ref ourNextId);
  }
}

public static class OcelLogger
{
  private const char Delimiter = ' ';


  public readonly struct OcelActivityCookie(string name, Guid activityId) : IDisposable
  {
    public void Dispose()
    {
      OcelEventsSource.Instance.OcelActivityEnd(activityId, name);
    }
  }


  public static void LogObjectAllocated<T>(T obj, string? category = null) where T : class
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.ObjectAllocated(GetObjectId(obj), category, string.Empty);
  }

  public static void LogObjectAllocated(long objectId, string? category = null)
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.ObjectAllocated(objectId, category, string.Empty);
  }

  public static void LogObjectConsumed<T>(T obj, string? category = null)
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.ObjectConsumed(GetObjectId(obj), category, string.Empty);
  }

  public static void LogObjectConsumed(long objectId, string? category = null)
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.ObjectConsumed(objectId, category, string.Empty);
  }

  public static void LogObjectConsumedWithProduce(long objectId, string? category = null, params ulong[] relatedObjectIds)
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.ObjectConsumedWithProduce(objectId, category, string.Join(Delimiter, relatedObjectIds), string.Empty);
  }

  public static void LogObjectConsumedWithProduce<T>(T obj, string? category = null, params T[] relatedObjects)
  {
    if (!IsEnabled()) return;

    var relatedObjectIds = JoinObjectsIds(relatedObjects.Select(GetObjectId));
    OcelEventsSource.Instance.ObjectConsumedWithProduce(GetObjectId(obj), category, relatedObjectIds, string.Empty);
  }

  public static void LogMergedObjectAllocated(long objectId, string? category = null, params long[] relatedObjectIds)
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.MergedObjectAllocated(objectId, category, JoinObjectsIds(relatedObjectIds), string.Empty);
  }

  public static void LogMergedObjectAllocated<T>(T obj, string? category = null, params T[] relatedObjects)
  {
    if (!IsEnabled()) return;

    var relatedObjectIds = JoinObjectsIds(relatedObjects.Select(GetObjectId));
    OcelEventsSource.Instance.MergedObjectAllocated(GetObjectId(obj), category, relatedObjectIds, string.Empty);
  }

  private static string JoinObjectsIds(IEnumerable<long> objectIds) => string.Join(Delimiter, objectIds);

  private static long GetObjectId<T>(T obj) => obj switch
  {
    OcelObjectBase @base => @base.Id,
    _ => RuntimeHelpers.GetHashCode(obj)
  };

  private static bool IsEnabled() => OcelEventsSource.Instance.IsEnabled();

  public static void LogGloballyAttachedObject<T>(T obj, string activityName, string? category = null) where T : class
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.OcelGloballyAttachedEvent(GetObjectId(obj), activityName, category, string.Empty);
  }

  public static OcelActivityCookie StartOcelActivity(string name)
  {
    var activityId = Guid.NewGuid();
    OcelEventsSource.Instance.OcelActivityBegin(activityId, name);
    return new OcelActivityCookie(name, activityId);
  }
}