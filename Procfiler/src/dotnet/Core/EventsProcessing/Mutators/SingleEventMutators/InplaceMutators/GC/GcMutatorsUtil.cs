using Core.Utils;

namespace Core.EventsProcessing.Mutators.SingleEventMutators.InplaceMutators.GC;

public static class GcMutatorsUtil
{
  public static string TransformGcReason(string reason, IProcfilerLogger logger) => reason switch
  {
    "AllocSmall" => "ALLOC_SMALL",
    "Induced" => "INDUCED",
    "LowMemory" => "LOW_MEMORY",
    "Empty" => "EMPTY",
    "AllocLarge" => "ALLOC_LARGE",
    "OutOfSpaceSOH" => "OOS_SOH",
    "OutOfSpaceLOH" => "OOS_LOH",
    "InducedNotForced" => "INDUCED_NOT_FORCED",
    "Internal" => "INTERNAL",
    "InducedLowMemory" => "INDUCED_LOW_MEMORY",
    "InducedCompacting" => "INDUCED_COMPACTING",
    "LowMemoryHost" => "LOW_MEMORY_HOST",
    "PMFullGC" => "PM_FULL_GC",
    "LowMemoryHostBlocking" => "LOW_MEMORY_HOST_BLOCKING",
    _ => MutatorsUtil.CreateUnknownEventNamePartAndLog(reason, logger)
  };

  public static string TransformGcType(string type, IProcfilerLogger logger) => type switch
  {
    "NonConcurrentGC" => "NC_GC",
    "BackgroundGC" => "B_GC",
    "ForegroundGC" => "F_GC",
    _ => MutatorsUtil.CreateUnknownEventNamePartAndLog(type, logger)
  };
}