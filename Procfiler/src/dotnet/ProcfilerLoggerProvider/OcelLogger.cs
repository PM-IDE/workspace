using System.Runtime.CompilerServices;
using ProcfilerEventSources;

namespace ProcfilerLoggerProvider;

public static class OcelLogger
{
  public readonly struct OcelActivityCookie(string name, Guid activityId) : IDisposable
  {
    public void Dispose()
    {
      OcelEventsSource.Instance.OcelActivityEnd(activityId, name);
    }
  }

  public readonly struct OcelActivitiesCookie(Guid batchId, string joinedNames) : IDisposable
  {
    public void Dispose()
    {
      OcelEventsSource.Instance.OcelActivitiesEnd(batchId, joinedNames);
    }
  }


  public static void LogObject<T>(T obj, string? category = null) where T : class
  {
    OcelEventsSource.Instance.OcelEvent(RuntimeHelpers.GetHashCode(obj), category, string.Empty);
  }

  public static void LogAttachedObject<T>(T obj, string activityName, string? category = null) where T : class
  {
    OcelEventsSource.Instance.OcelAttachedToActivityEvent(RuntimeHelpers.GetHashCode(obj), activityName, category, string.Empty);
  }

  public static OcelActivityCookie StartOcelActivity(string name)
  {
    var activityId = Guid.NewGuid();
    OcelEventsSource.Instance.OcelActivityBegin(activityId, name);
    return new OcelActivityCookie(name, activityId);
  }

  public static OcelActivitiesCookie StartOcelActivities(string[] names)
  {
    var batchId = Guid.NewGuid();
    var joinedNames = string.Join(';', names);

    OcelEventsSource.Instance.OcelActivitiesBegin(batchId, joinedNames);

    return new OcelActivitiesCookie(batchId, joinedNames);
  }
}