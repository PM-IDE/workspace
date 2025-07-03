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


  public static void LogObject<T>(T obj, string? category = null) where T : class
  {
    OcelEventsSource.Instance.OcelEvent(RuntimeHelpers.GetHashCode(obj), category, string.Empty);
  }

  public static OcelActivityCookie StartOcelActivity(string name)
  {
    var activityId = Guid.NewGuid();
    OcelEventsSource.Instance.OcelActivityBegin(activityId, name);
    return new OcelActivityCookie(name, activityId);
  }
}