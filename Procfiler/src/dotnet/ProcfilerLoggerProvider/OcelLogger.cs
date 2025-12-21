using System.Runtime.CompilerServices;
using ProcfilerEventSources;

namespace ProcfilerLoggerProvider;

public abstract class OcelObjectBase
{
  private static long ourNextId;

  public long Id { get; }
  public virtual string? Type => GetType().FullName;


  protected OcelObjectBase()
  {
    Id = Interlocked.Increment(ref ourNextId);
  }
}

public readonly struct OcelObjectDto(long objectId, string? type = null)
{
  public long Id => objectId;
  public string? Type => type;
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


  public static void LogObjectAllocated<T>(T obj) where T : class
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.OcelObjectAllocated(GetObjectId(obj), GetObjectType(obj), string.Empty);
  }

  public static void LogObjectAllocatedRaw(in OcelObjectDto dto)
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.OcelObjectAllocated(dto.Id, dto.Type, string.Empty);
  }

  public static void LogObjectConsumed<T>(T obj, string? category = null)
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.OcelObjectConsumed(GetObjectId(obj), category, string.Empty);
  }

  public static void LogObjectConsumedRaw(in OcelObjectDto dto)
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.OcelObjectConsumed(dto.Id, dto.Type, string.Empty);
  }

  public static void LogConsumeProduceRaw(long objectId, params OcelObjectDto[] relatedObjectIds)
  {
    if (!IsEnabled()) return;

    var relatedIds = JoinObjectsIds(relatedObjectIds.Select(o => o.Id));
    var relatedTypes = JoinObjectTypes(relatedObjectIds.Select(o => o.Type));

    OcelEventsSource.Instance.OcelConsumeProduce(objectId, relatedIds, relatedTypes, string.Empty);
  }

  public static void LogConsumeProduce<T>(T obj, params T[] relatedObjects)
  {
    if (!IsEnabled()) return;

    var relatedObjectIds = JoinObjectsIds(relatedObjects.Select(GetObjectId));
    var relatedObjectTypes = JoinObjectTypes(relatedObjectIds.Select(GetObjectType));

    OcelEventsSource.Instance.OcelConsumeProduce(GetObjectId(obj), relatedObjectIds, relatedObjectTypes, string.Empty);
  }

  public static void LogMergeAllocateRaw(OcelObjectDto allocatedObject, params long[] relatedObjectIds)
  {
    if (!IsEnabled()) return;

    OcelEventsSource.Instance.OcelMergeAllocate(allocatedObject.Id, allocatedObject.Type, JoinObjectsIds(relatedObjectIds), string.Empty);
  }

  public static void LogMergeAllocate<T>(T obj, params T[] relatedObjects)
  {
    if (!IsEnabled()) return;

    var relatedObjectIds = JoinObjectsIds(relatedObjects.Select(GetObjectId));
    OcelEventsSource.Instance.OcelMergeAllocate(GetObjectId(obj), GetObjectType(obj), relatedObjectIds, string.Empty);
  }

  private static string JoinObjectsIds(IEnumerable<long> objectIds) => string.Join(Delimiter, objectIds);
  private static string JoinObjectTypes(IEnumerable<string?> types) => string.Join(Delimiter, types);

  private static long GetObjectId<T>(T obj) => obj switch
  {
    OcelObjectBase @base => @base.Id,
    _ => RuntimeHelpers.GetHashCode(obj)
  };

  private static string? GetObjectType<T>(T obj) => obj switch
  {
    OcelObjectBase @base => @base.Type,
    _ => obj?.GetType().FullName
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