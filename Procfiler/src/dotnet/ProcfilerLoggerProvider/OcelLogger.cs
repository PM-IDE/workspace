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


  public static long? LogObjectAllocated<T>(T obj) where T : class
  {
    if (!IsEnabled()) return null;

    var id = GetObjectId(obj);
    OcelEventsSource.Instance.OcelObjectAllocated(id, GetObjectType(obj), string.Empty);

    return id;
  }

  public static long? LogObjectAllocatedRaw(in OcelObjectDto dto)
  {
    if (!IsEnabled()) return null;

    OcelEventsSource.Instance.OcelObjectAllocated(dto.Id, dto.Type, string.Empty);

    return dto.Id;
  }

  public static long? LogObjectConsumed<T>(T obj, string? category = null)
  {
    if (!IsEnabled()) return null;

    var id = GetObjectId(obj);
    OcelEventsSource.Instance.OcelObjectConsumed(id, category, string.Empty);

    return id;
  }

  public static long? LogObjectConsumedRaw(in OcelObjectDto dto)
  {
    if (!IsEnabled()) return null;

    OcelEventsSource.Instance.OcelObjectConsumed(dto.Id, dto.Type, string.Empty);

    return dto.Id;
  }

  public static long? LogConsumeProduceRaw(long objectId, params OcelObjectDto[] relatedObjectIds)
  {
    if (!IsEnabled()) return null;

    var relatedIds = JoinObjectsIds(relatedObjectIds.Select(o => o.Id));
    var relatedTypes = JoinObjectTypes(relatedObjectIds.Select(o => o.Type));

    OcelEventsSource.Instance.OcelConsumeProduce(objectId, relatedIds, relatedTypes, string.Empty);

    return objectId;
  }

  public static long? LogConsumeProduce<T>(T obj, params T[] relatedObjects)
  {
    if (!IsEnabled()) return null;

    var relatedObjectIds = JoinObjectsIds(relatedObjects.Select(GetObjectId));
    var relatedObjectTypes = JoinObjectTypes(relatedObjectIds.Select(GetObjectType));

    var id = GetObjectId(obj);
    OcelEventsSource.Instance.OcelConsumeProduce(id, relatedObjectIds, relatedObjectTypes, string.Empty);

    return id;
  }

  public static long? LogMergeAllocateRaw(OcelObjectDto allocatedObject, params long[] relatedObjectIds)
  {
    if (!IsEnabled()) return null;

    OcelEventsSource.Instance.OcelMergeAllocate(
      allocatedObject.Id, allocatedObject.Type, JoinObjectsIds(relatedObjectIds), string.Empty);

    return allocatedObject.Id;
  }

  public static long? LogMergeAllocate<T>(T obj, params T[] relatedObjects)
  {
    if (!IsEnabled()) return null;

    var relatedObjectIds = JoinObjectsIds(relatedObjects.Select(GetObjectId));
    var id = GetObjectId(obj);

    OcelEventsSource.Instance.OcelMergeAllocate(id, GetObjectType(obj), relatedObjectIds, string.Empty);

    return id;
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