using Core.Constants.TraceEvents;
using Core.Container;
using Procfiler.Core.EventRecord.EventsCollection;
using Procfiler.Core.EventsProcessing.Filters.Core;

namespace Procfiler.Core.EventsProcessing.Filters;

[EventFilter]
public class OnlyKnownEventsFilterer : IEventsFilter
{
  private static readonly HashSet<string> ourAllowedEvents =
  [
    TraceEventsConstants.GcSampledObjectAllocation,
    TraceEventsConstants.GcCreateSegment,
    TraceEventsConstants.GcFinalizersStart,
    TraceEventsConstants.GcFinalizersStop,
    TraceEventsConstants.GcSuspendEeStart,
    TraceEventsConstants.GcSuspendEeStop,
    TraceEventsConstants.GcRestartEeStart,
    TraceEventsConstants.GcRestartEeStop,
    TraceEventsConstants.GcSetGcHandle,
    TraceEventsConstants.GcDestroyGcHandle,
    TraceEventsConstants.GcStart,
    TraceEventsConstants.GcStop,
    TraceEventsConstants.GcTriggered,
    TraceEventsConstants.GcFinalizeObject,
    TraceEventsConstants.GcPinObjectAtGcTime,
    TraceEventsConstants.BgcStart,
    TraceEventsConstants.Bgc1StNonCondStop,
    TraceEventsConstants.BgcRevisit,
    TraceEventsConstants.BgcDrainMark,
    TraceEventsConstants.Bgc1StConStop,
    TraceEventsConstants.Bgc2NdNonConStart,
    TraceEventsConstants.Bgc2NdNonConStop,
    TraceEventsConstants.Bgc2NdConStart,
    TraceEventsConstants.Bgc2NdConStop,
    TraceEventsConstants.Bgc1StSweepEnd,
    TraceEventsConstants.BgcOverflow,
    TraceEventsConstants.BgcAllocWaitStart,
    TraceEventsConstants.BgcAllocWaitStop,
    TraceEventsConstants.GcFullNotify,
    TraceEventsConstants.GcCreateConcurrentThread,
    TraceEventsConstants.GcTerminateConcurrentThread,
    TraceEventsConstants.GcLohCompact,

    TraceEventsConstants.ContentionStart,
    TraceEventsConstants.ContentionStop,

    TraceEventsConstants.ExceptionStart,
    TraceEventsConstants.ExceptionStop,
    TraceEventsConstants.ExceptionCatchStart,
    TraceEventsConstants.ExceptionCatchStop,
    TraceEventsConstants.ExceptionFinallyStart,
    TraceEventsConstants.ExceptionFinallyStop,
    TraceEventsConstants.ExceptionFilterStart,
    TraceEventsConstants.ExceptionFilterStop,

    TraceEventsConstants.BufferAllocated,
    TraceEventsConstants.BufferRented,
    TraceEventsConstants.BufferReturned,
    TraceEventsConstants.BufferTrimmed,
    TraceEventsConstants.BufferTrimPoll,

    TraceEventsConstants.AssemblyLoaderAppDomainAssemblyResolveHandlerInvoked,
    TraceEventsConstants.AssemblyLoaderAssemblyLoadFromResolveHandlerInvoked,
    TraceEventsConstants.AssemblyLoaderStart,
    TraceEventsConstants.AssemblyLoaderStop,
    TraceEventsConstants.AssemblyLoaderKnownPathProbed,
    TraceEventsConstants.AssemblyLoaderResolutionAttempted,

    TraceEventsConstants.LoaderAppDomainLoad,
    TraceEventsConstants.LoaderAppDomainUnload,
    TraceEventsConstants.LoaderAssemblyLoad,
    TraceEventsConstants.LoaderAssemblyUnload,
    TraceEventsConstants.LoaderModuleLoad,
    TraceEventsConstants.LoaderModuleUnload,
    TraceEventsConstants.LoaderDomainModuleLoad,

    TraceEventsConstants.MethodInliningFailed,
    TraceEventsConstants.MethodInliningSucceeded,
    TraceEventsConstants.MethodLoadVerbose,
    TraceEventsConstants.MethodUnloadVerbose,
    TraceEventsConstants.MethodTailCallFailed,
    TraceEventsConstants.MethodTailCallSucceeded,
    TraceEventsConstants.MethodR2RGetEntryPoint,
    TraceEventsConstants.MethodR2RGetEntryPointStart,
    TraceEventsConstants.MethodMemoryAllocatedForJitCode,

    TraceEventsConstants.TaskExecuteStart,
    TraceEventsConstants.TaskExecuteStop,
    TraceEventsConstants.TaskWaitSend,
    TraceEventsConstants.TaskWaitStop,
    TraceEventsConstants.TaskScheduledSend,
    TraceEventsConstants.TaskWaitContinuationStarted,
    TraceEventsConstants.TaskWaitContinuationComplete,
    TraceEventsConstants.AwaitTaskContinuationScheduledSend,
    TraceEventsConstants.IncompleteAsyncMethod,

    TraceEventsConstants.ThreadPoolWorkerThreadStart,
    TraceEventsConstants.ThreadPoolWorkerThreadStop,
    TraceEventsConstants.ThreadPoolWorkerThreadRetirementStart,
    TraceEventsConstants.ThreadPoolWorkerThreadRetirementStop,
    TraceEventsConstants.ThreadPoolWorkerThreadAdjustmentStats,
    TraceEventsConstants.ThreadPoolWorkerThreadAdjustmentAdjustment,
    TraceEventsConstants.ThreadPoolWorkerThreadAdjustmentSample,
    TraceEventsConstants.IoThreadCreate,
    TraceEventsConstants.IoThreadTerminate,
    TraceEventsConstants.IoThreadRetire,
    TraceEventsConstants.IoThreadUnRetire,
    TraceEventsConstants.ThreadPoolDequeueWork,
    TraceEventsConstants.ThreadPoolEnqueueWork,
    TraceEventsConstants.ThreadPoolWorkerThreadWait,

    TraceEventsConstants.AppDomainResourceManagementThreadCreated,

    TraceEventsConstants.RequestStart,
    TraceEventsConstants.RequestStop,
    TraceEventsConstants.RequestFailed,
    TraceEventsConstants.ConnectionEstablished,
    TraceEventsConstants.ConnectionClosed,
    TraceEventsConstants.RequestLeftQueue,
    TraceEventsConstants.RequestHeadersStart,
    TraceEventsConstants.RequestHeadersStop,
    TraceEventsConstants.RequestContentStart,
    TraceEventsConstants.RequestContentStop,
    TraceEventsConstants.ResponseContentStart,
    TraceEventsConstants.ResponseContentStop,
    TraceEventsConstants.ResponseHeadersStart,
    TraceEventsConstants.ResponseHeadersStop,

    TraceEventsConstants.ConnectStart,
    TraceEventsConstants.ConnectStop,
    TraceEventsConstants.ConnectFailed,
    TraceEventsConstants.AcceptStart,
    TraceEventsConstants.AcceptStop,
    TraceEventsConstants.AcceptFailed
  ];


  public IEnumerable<string> AllowedEventsNames => ourAllowedEvents;

  public void Filter(IEventsCollection events)
  {
    foreach (var (ptr, eventRecord) in events)
    {
      if (!ourAllowedEvents.Contains(eventRecord.EventClass))
      {
        events.Remove(ptr);
      }
    }
  }
}