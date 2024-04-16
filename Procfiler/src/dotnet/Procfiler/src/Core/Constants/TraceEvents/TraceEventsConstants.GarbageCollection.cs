namespace Procfiler.Core.Constants.TraceEvents;

public static partial class TraceEventsConstants
{
  public const string GcStart = "GC/Start";
  public const string GcStartReason = CommonReason;
  public const string GcStartType = "Type";

  public const string GcSuspendEeStart = "GC/SuspendEEStart";
  public const string GcSuspendEeStartReason = CommonReason;
  public const string GcSuspendEeStop = "GC/SuspendEEStop";

  public const string GcTriggered = "GC/Triggered";

  public const string GcSetGcHandle = "GC/SetGCHandle";

  public const string GcSampledObjectAllocation = "GC/SampledObjectAllocation";
  public const string GcSampledObjectAllocTypeId = "TypeID";
  public const string GcSampledObjectAllocationTypeName = CommonTypeName;

  public const string GcFinalizeObject = "GC/FinalizeObject";

  public const string GcPinObjectAtGcTime = "GC/PinObjectAtGCTime";
  public const string GcAllocationTick = "GC/AllocationTick";
  public const string GcGenerationRange = "GC/GenerationRange";
  public const string GcBulkNode = "GC/BulkNode";
  public const string GcBulkRootConditionalWeakTableElementEdge = "GC/BulkRootConditionalWeakTableElementEdge";
  public const string GcBulkRootEdge = "GC/BulkRootEdge";
  public const string GcBulkRootStaticVar = "GC/BulkRootStaticVar";
  public const string GcBulkSurvivingObjectRanges = "GC/BulkSurvivingObjectRanges";
  public const string GcGlobalHeapHistory = "GC/GlobalHeapHistory";
  public const string GcHeapStats = "GC/HeapStats";
  public const string GcMarkWithType = "GC/MarkWithType";
  public const string GcPerHeapHistory = "GC/PerHeapHistory";
  public const string GcBulkEdge = "GC/BulkEdge";
  public const string GcCreateSegment = "GC/CreateSegment";
  public const string GcFinalizersStart = "GC/FinalizersStart";
  public const string GcFinalizersStop = "GC/FinalizersStop";
  public const string GcRestartEeStart = "GC/RestartEEStart";
  public const string GcRestartEeStop = "GC/RestartEEStop";
  public const string GcDestroyGcHandle = "GC/DestroyGCHandle";
  public const string GcStop = "GC/Stop";
  public const string GcLohCompact = "GC/LOHCompact";

  public const string BgcStart = "GC/BGCStart";
  public const string Bgc1StNonCondStop = "GC/BGC1stNonCondStop";
  public const string BgcRevisit = "GC/BGCRevisit";
  public const string BgcDrainMark = "GC/BGCDrainMark";
  public const string Bgc1StConStop = "GC/BGC1stConStop";
  public const string Bgc2NdNonConStart = "GC/BGC2ndNonConStart";
  public const string Bgc2NdNonConStop = "GC/BGC2ndNonConStop";
  public const string Bgc2NdConStart = "GC/BGC2ndConStart";
  public const string Bgc2NdConStop = "GC/BGC2ndConStop";
  public const string BgcPlanStop = "GC/BGCPlanStop";
  public const string Bgc1StSweepEnd = "GC/BGC1stSweepEnd";
  public const string BgcOverflow = "GC/BGCOverflow";
  public const string BgcAllocWaitStart = "GC/BGCAllocWaitStart";
  public const string BgcAllocWaitStop = "GC/BGCAllocWaitStop";
  public const string GcFullNotify = "GC/FullNotify";
  public const string GcDecision = "GC/Decision";
  public const string GcCreateConcurrentThread = "GC/CreateConcurrentThread";
  public const string GcTerminateConcurrentThread = "GC/TerminateConcurrentThread";

  public const string GcCount = "Count";
}