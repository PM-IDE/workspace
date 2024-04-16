The classification, description and useful attributes of CLR events, which can be used later in this project.

# Table of contents:

1. [Garbage collection (GC) events](#garbage-collection-gc-events)
   1. [GCAllocationTick](#gcgcallocationtick)
   2. [SampledObjectAllocation](#gcsampledobjectallocation)
   3. [CreateSegment](#gccreatesegment)
   4. [Finalizers(Start/Stop)](#gcfinalizersstartstop)
   5. [SuspendEE(Start/Stop)](#gcsuspendeestartstop)
   6. [RestartEE(Start/Stop)](#gcrestarteestartstop)
   7. [(Set/Destroy)GCHandle](#gcsetdestroygchandle)
   8. [GC(Start/End)](#gcgcstartend)
   9. [Triggered](#gcgctriggered)
   10. [Finalize Object](#gcfinalizeobject)
   11. [Pin object at GC time](#gcpinobjectatgctime)
   12. [GC/BGCStart](#gcbgcstart)
   13. [GC/BGC1stNonCondStop](#gcbgc1stnoncondstop)
   14. [GC/BGCRevisit](#gcbgcrevisit)
   15. [GC/BGCDrainMark](#gcbgcdrainmark)
   16. [GC/BGC1stConStop](#gcbgc1stconstop)
   17. [GC/BGC2ndNonConStart](#gcbgc2ndnonconstart)
   18. [GC/BGC2ndNonConStop](#gcbgc2ndnonconstop)
   19. [GC/BGC2ndConStart](#gcbgc2ndconstart)
   20. [GC/BGC2ndConStop](#gcbgc2ndconstop)
   21. [GC/BGCPlanStop](#gcbgcplanstop)
   22. [GC/BGC1stSweepEnd](#gcbgc1stsweepend)
   23. [GC/BGCOverflow](#gcbgcoverflow)
   24. [GC/BGCAllocWaitStart](#gcbgcallocwaitstart)
   25. [GC/BGCAllocWaitStop](#gcbgcallocwaitstop)
   26. [GC/FullNotify](#gcfullnotify)
   27. [GC/Decision](#gcdecision)
   28. [GC/(Create/Terminate)ConcurrentThread](#gc--terminatecreate--concurrentthread)
   29. [GC/LOHCompact](#gclohcompact)
2. [Contention](#contention-events)
   1. [Contention start](#contention-start)
   2. [Contention end](#contention-end)
3. [Exceptions](#exception-events)
   1. [Exception(Start/Stop)](#exceptionstartstop)
   2. [ExceptionCatch(Start/Stop)](#exceptioncatchstartstop)
   3. [ExceptionFinally(Start/Stop)](#exceptionfinallystartstop)
   4. [ExceptionFilter(Start/Stop)](#exceptionfilterstartstop)
4. [Array-pool buffer events](#array-pool-buffer-events)
   1. [Buffer allocated](#buffer-allocated-event)
   2. [Buffer rented](#buffer-rented-event)
   3. [Buffer returned](#buffer-returned-event)
   4. [Buffer trimmed](#buffer-trimmed-event)
   5. [Buffer trim poll](#buffer-trim-poll)
5. [Assembly loader events](#assembly-loader-events)
   1. [AssemblyLoader(Start/Stop)](#assemblyloaderstartstop)
   2. [AssemblyLoader/ResolutionAttempted](#assemblyloaderresolutionattempted)
   3. [AssemblyLoader/KnownPathProbed](#assemblyloaderknownpathprobed)
   4. [AssemblyLoader/AppDomainAssemblyResolveHandlerInvoked](#assemblyloaderappdomainassemblyresolvehandlerinvoked)
   5. [AssemblyLoader/AssemblyLoadContextResolvingHandlerInvoked](#assemblyloaderassemblyloadcontextresolvinghandlerinvoked)
6. [Loader events](#loader-events)
   1. [AppDomain load/unload](#loaderappdomainloadunloaddcstop-events)
   2. [Assembly load/unload](#loaderassemblyloadunload)
   3. [Assembly DC stop](#loaderassemblydcstop)
   4. [Module load/unload](#loadermoduleloadunloaddcstop)
   5. [Domain module load](#loaderdomainmoduleload-or-loaderdomainmoduledcstop)
7. [Method events](#method-events)
   1. [Inlining failed](#methodinliningfailed)
   2. [Inlining succeeded](#methodinliningsucceeded)
   3. [Memory allocated for jit code](#methodmemoryallocatedforjitcode)
   4. [Jitting started](#methodjittingstarted)
   5. [Method (load/unload)](#methodloadunloadverbose)
   6. [R2R get entry point start](#methodr2rgetentrypointstart)
   7. [R2R get entry point finished](#methodr2rgetentrypoint)
   8. [Tail call succeeded](#methodtailcallsucceeded)
   9. [Tail call failed](#methodtailcallfailed)
8. [Tasks/Thread/ThreadPool events](#tasksthreadthreadpool-events)
   1. [Task execution start](#taskexecutestart)
   2. [Task execute stop](#taskexecutestop)
   3. [Task wait start](#taskwaitsend)
   4. [Task wait stop](#taskwaitstop)
   5. [Task scheduled](#taskscheduledsend)
   6. [Task-wait continuation started](#taskwaitcontinuationstarted)
   7. [Task-wait continuation complete](#taskwaitcontinuationcomplete)
   8. [Await task continuation scheduled](#awaittaskcontinuationscheduled)
   9. [Incomplete async method](#incompleteasyncmethod)
   10. [Thread pool worker thread (start/stop)](#threadpoolworkerthreadstartstop)
   11. [Thread pool worker thread retirement (start/stop)](#threadpoolworkerthreadretirementstartstop)
   12. [Thread pool worker thread adjustment sample](#threadpoolworkerthreadadjustmentsample)
   13. [Thread pool worker thread adjustment adjustment](#threadpoolworkerthreadadjustmentadjustment)
   14. [IO thread (create/terminate)](#iothread-createterminate)
   15. [IO thread (retire/unretire)](#iothread-retireunretire)
   16. [ThreadPoolDequeueWork](#threadpooldequeuework)
   17. [ThreadPoolEnqueueWork](#threadpoolenqueuework)
   18. [ThreadPoolWorkerThread/Wait](#threadpoolworkerthreadwait)
9. [Native events](#native-events)
   1. [Thread created](#appdomainresourcemanagementthreadcreated)
10. [HTTP events](#http-events)
    1. [Request start/stop](#requeststartstop)
    2. [Request failed](#requestfailed)
    3. [Connection (established/closed)](#connectionestablishedclosed)
    4. [Request left queue](#requestleftqueue)
    5. [Request headers (start/stop)](#requestheadersstartstop)
    6. [Request content (start/stop)](#requestcontentstartstop)
    7. [Response headers (start/stop)](#responseheadersstartstop)
    8. [Response content (start/stop)](#responseheadersstartstop)
11. [Sockets](#sockets)
    1. [Connect start](#connect-stop)
    2. [Connect stop](#connect-stop)
    3. [Connect failed](#connect-failed)
    4. [Accept start](#accept-start)
    5. [Accept stop](#accept-stop)
    6. [Accept failed](#accept-failed)


# Garbage collection (GC) events

## GC/GCAllocationTick

The event is fired in `gc_heap` class in the `coreclr`. The function to fire this event is called `fire_etw_allocation_event`, and used in the famous `gc.cpp` file.
There are two versions of this event, `V1` which is called when `FEATURE_NATIVEAOT` is enabled, and `V4` when `FEATURE_NATIVEAOT` is disabled.
The event has following initial attributes:
- Allocation amount
- The heap index (0 = Small object heap (soh), 1 = Large objects heap (loh), 2 = Pinned objects heap (poh), -1 = unknown, should never happen)
- Heap number, if multiple heaps is used, when the GC operates in server mode, there can one heap per thread.
- The address of last allocated object
- The size of an object

The event has following derived attributes:
- Managed thread id
- Stack trace id (if present)

This is NOT raised on every object allocation, there is a special limit `etw_allocation_tick`, which is set to `100 * 1024` bytes, so if our allocation is not incrementally exceeding the limit, the event will not be fired.

## GC/SampledObjectAllocation

This event is fired on EVERY object allocation, so its frequency is much higher, than the frequency of `GC/GCAllocationTick`.
The event is called in `gchelpers.cpp` in method `PublishObjectAndNotify`, after the object is allocated. Moreover, the profiler is
also notified about the allocated object.

This event contains following initial attributes:
- Address of allocated object
- Type ID
- Number of allocated objects
- Total size, which was allocated
- Clr instance ID (this attribute is present among almost all events, so it will be omitted in future)

This event contains following derived attributes:
- Managed thread id
- Stack trace id (if present)
- ProcfilerTypeName (which is resolved with the help of Type/BulkType event, from which we can create a map of TypeIDs into names)

## GC/CreateSegment

The event is fired when the segment is created (or, if we attached to already running process, the event can fired for each
already existing segment, to indicate their presence). The event is fired in several functions in `gc.cpp` file.

The event has following initial attributes:
- Address of created (existing) segment
- Size of created (exiting) segment
- Segment type (soh = 0, loh = 1, read_only_heap = 2, poh = 3)

## GC/Finalizers(Start/Stop)

The event indicates that finalizer thread started (stopped) finalizing all objets. The event is called from finalizer thread
The stop event has attribute:
- Count, which indicates how many objects were finalized

## GC/SuspendEE(Start/Stop)

The event which indicates that Execution Engine was suspended. This can happen because of GC, for example. The events are fired in
`gcrhenv.cpp`, in `SuspendEE` function. The event doesn't have any attributes, it just indicates start/end of EE suspension.

The GC/SuspendEEStart event has following initial attributes:
- Reason
  - Other
  - ForGC
  - ForAppDomainShutdown
  - ForCodePitching
  - ForShutdown
  - ForDebugger
  - ForGCPrep
  - ForDebuggerSweep
- Count - The GC count at the time. Usually, you would see a subsequent GC Start event after this, and its Count would be this Count + 1 as we increase the GC index during a garbage collection.


## GC/RestartEE(Start/Stop)

The event indicating that EE is restarted after suspension. The events are fired in `gcrhenv.cpp` in `RestartEE` function. In combination
with SuspendEE(Start/Stop) we can track EE suspensions.

## GC/(Set/Destroy)GCHandle

The event which indicates that GC handle was created (destroyed). The gc handle can be used to tell GC not to move managed objects (and not to collect them),
which are referenced from unmanaged code. The event called in `handletable.cpp`, in HndLogSetEvent.

SetGCHandle contains following attributes:
- ID of handle
- Oject ID
- Kind of handle (Weak, Strong, WeakShort, Pinned)
- Generation
- ID of application domain

SetGCHandle and DestroyGCHandle contains following derived attributes:
- Managed thread id
- Stacktrace id

DestroyGCHandle contains following attributes:
- ID of handle

## GC/GC(Start/End)

The event which indicates that Garbage Collection has started (ended).
The GCStart event is fired in `eventtrace.cpp` in `FireGcStart(ETW_GC_INFO * pGcInfo)` method. 

GCStart contains following initial attributes:
- Count - the nth garbage collection, so it is the index of GC?
- Reason - the reason why GC started. Can be 
  - GC_ALLOC_SOH - allocation in SOH
  - GC_INDUCED
  - GC_LOWMEMORY
  - GC_EMPTY
  - GC_ALLOC_LOH - allocation in LOH
  - GC_OOS_SOH - Out of spaces in SOH
  - GC_OOS_LOH - Out of spaces in LOH
  - GC_INDUCED_NOFORCE - called but not as blocking
  - GC_GCSTRESS - Stress testing (WTF?)
  - GC_LOWMEMORY_BLOCKING
  - GC_INDUCED_COMPACTING 
  - GC_LOWMEMORY_HOST
  - PMFullGC - ???
  - LowMemoryHostBlocking - ???
- Depth - the generation which is collected
- Type
  - NGC (Non Concurrent GC) - the default blocking GC
  - BGC - the background GC
  - FGC (Foreground GC) - the blocking GC during concurrent sweep of 2nd generation and LOH

GCStop event has Count and Depth attributes, so we can link the GCStart events with the GCEnd events.

## GC/GCTriggered

Fires from `gc.cpp` `GCHeap::GarbageCollectGeneration` method. indicates that garbage collection was triggered, but in contrast with
GCStart event provides information about Managed Thread Id and Stack Trace, so we can identify which managed thread caused GC.

Contains following initial attributes:
- Reason

and following derived:
- Managed Thread id
- Stack trace id

## GC/FinalizeObject

The event is fired when finalizing objects.

Initial attributes:
- Type name
- Object ID
- Type ID

## GC/PinObjectAtGCTime

Initial attributes:
- Object ID
- Handle ID
- Type name

## GC/BGCStart

Indicating start of background GC.
The event is fired on "-1" thread, it doesn't have a stacktrace.
The event is fired from `gc_heap::gc1()` method.
Derived attributes:
- StackTraceId = -1
- ManagedThreadId = -1

## GC/BGC1stNonCondStop

Indicates that initial blocking marking is finished. During initial marking the GC roots are found, which are then used
in concurrent marking stage.
Derived attributes:
- StackTraceId = -1
- ManagedThreadId = -1

## GC/BGCRevisit

The event is fired during "revisit" stage of concurrent marking. The point of "revisit" stage is in the following:
as we are running concurrent marking, during this process user code can modify references, so we need to keep track of
all modifications during concurrent mark stage. Different mechanism can be used to track those modifications, for example
OS specific (WriteWatch on Windows): 
`This mechanism has page-wide granularity so even a single modified object will invalidate the whole 4kB page`
(from `Pro .NET Memory management`)
or similar custom implementation on non-Windows systems.
The event is fired in `gc_heap::revisit_written_pages` method in `gc.cpp` file.
The event is also fired during the stop-the-world marking phase after concurrent marking in GC.

## GC/BGCDrainMark

The event contains information how many objects are in the "work list". Work list is one of the sources of objects which should be marked.
Work list is created on the first step of Background GC, during the first stop-the-world stage. Objects from stack
and finalization queue are present in the work list. 
This event is fired from `gc_heap::background_drain_mark_list` in the `gc.cpp` file.

Initial attributes:
- Objects - the number of object in work list

Derived attributes:
- StackTraceId = -1
- ManagedThreadId = -1

## GC/BGC1stConStop

Indicates that concurrent marking has finished. The event name can be explained as "1st concurrent stop", i.e. the end
of concurrent marking phase.
The event is fired from `gc_heap::background_mark_phase` method in `gc.cpp`
Derived attributes:
- StackTraceId = -1
- ManagedThreadId = -1

## GC/BGC2ndNonConStart

The event is fired before the non-concurrent marking starts. After we finished the concurrent marking, we still need to
get the real picture of marking objects. The point of this phase is that a lot of work is already done during
concurrent marking, so we expect this pause to be short, as a lot of work can be done incrementally, taking into account
the results of concurrent marking.
`The revisiting of the roots is only to ensure that there are no new reachable objects available`
(from `Pro .NET Memory management`)
The event is fired in `gc_heap::background_mark_phase` method in `gc.cpp`

Derived attributes:
- StackTraceId = -1
- ManagedThreadId = -1

## GC/BGC2ndNonConStop

The event indicates the end of the stop-the-world marking phase (start of which is indicated by `GC/BGC2ndNonConStart`).
The event is fired in `gc_heap::background_sweep` method in `gc.cpp`.
The `background_sweep` function is called straight after `gc_heap::gc1`.

Derived attributes:
- StackTraceId = -1
- ManagedThreadId = -1

## GC/BGC2ndConStart

The event is fired in `gc_heap::background_sweep` method in `gc.cpp`.
The event indicates the start of background sweep of SOH and LOH (POH also? Well there is a new FOH as well, 
but for frozen objects heap there should be no collection as objects can't become unreachable, as they are frozen. 
Well that's just my thoughts, they should be checked).

Derived attributes:
- StackTraceId = -1
- ManagedThreadId = -1

## GC/BGC2ndConStop

The event is also fired in `gc_heap::background_sweep` method in `gc.cpp`. This event indicates the end of background sweep
(start of which is indicated by `GC/BGC2ndConStart`)

Derived attributes:
- StackTraceId = -1
- ManagedThreadId = -1

## GC/BGCPlanStop

Seems like this event is not used, I haven't found a place where it is fired in source code

## GC/BGC1stSweepEnd

The event indicates that background sweep for max_generation is over. 
The event is fired in `gc_heap::background_sweep` method in `gc.cpp`

Derived attributes:
- StackTraceId = -1
- ManagedThreadId = -1

## GC/BGCOverflow

The is fired in `gc_heap::background_process_mark_overflow_internal` in `gc.cpp` file.
Seems like this event handles overflow in mark array(?). The method `background_process_mark_overflow_internal` is not called
anywhere in `gc.cpp` file, so maybe this event is unused for now.
TODO: check this

## GC/BGCAllocWaitStart

During background sweep the allocations in LOH are paused, so you can't allocate anything in LOH during background sweep.
This event indicates the start of this period.
The event is fired from `gc_heap::fire_alloc_wait_event_begin` function.
TODO: find where this method is called.

Derived attributes:
- StackTraceId = -1
- ManagedThreadId = -1

## GC/BGCAllocWaitStop

Event indicating that suppression of LOH allocations has finished (the start of which is indicated by `GC/BGCAllocWaitStart`.
Derived attributes:
- StackTraceId = -1
- ManagedThreadId = -1

## GC/FullNotify

The event is notifying about full garbage collection. The event is fired from:
`gc_heap::check_for_full_gc`, `gc_heap::allocate_uoh`, `gc_heap::allocate_soh`.

Initial attributes:
- GenNumber - the index of generation
- IsAlloc - was full GC triggered because of allocation budget (`number of bytes to be allocated into a generation before a GC is triggered on that generation`, from `Pro .NET memory management`)

## GC/Decision

Seems like this event is not used, I haven't found a place where it is fired in source code

## GC/(Terminate/Create)ConcurrentThread

The event is fired when concurrent thread (or `bgc thread`, that is how it is called in the code) is created or terminated.
The creation event is fired in `gc_heap::prepare_bgc_thread`.
The termination event is fired in the end of concurrent thread's procedure, in `gc_heap::bgc_thread_function`.

## GC/LOHCompact

For some reason this event is not present in TraceEvent library, but there is a code which fires it in `gc.cpp` at `gc_heap::fire_pevents`
The time can be 0, if we don't compact LOH.

The event should contains following information:
- time_plan - time it takes to plan
- time_compact - time it takes to compact
- time_relocate - time it takes to relocate
- total_refs
- zero_refs

# Contention events

## Contention Start

Fired when e.g. System.Threading.Monitor tries to acquire a lock, or some native locks do so. The event is fired in `syncblk.cpp` in
`AwareLock::EnterEpilogHelper` method. Both contention start and stop events are fired here.

The event contains following initial attributes:
- Contention Flags = 0 (for managed) or 1 (for unmanaged)

The event contains following derived attributes:
- Managed thread Id
- Stack trace Id

## Contention End

The event indicating that contention has ended. Also fired from `AwareLock::EnterEpilogHelper`.

The event contains following initial attributes:
- Duration (in nano-seconds)
- Contention Flags (Managed or Unmanaged)

# Exception events

## Exception(Start/Stop)

Exception start event provides a lot of information about exception handling.

The Exception/Start contains following initial attributes
- ExceptionType: the FQN of exception type
- Exception message: the message of exception
- ExceptionEIP: the instruction pointer where exception has occured
- Exception HRESULT
- ExceptionFlags - (None = 0, HasInnerException = 0x1, Nested = 0x2, ReThrown = 0x4, CorruptedState = 0x8, CLSCompliant = 0x10, this is flags enum)

The Exception/Start and Exception/Stop contains following derived attributes:
- Managed thread ID
- Stack trace ID

## ExceptionCatch(Start/Stop)

The ExceptionCatch(Start/Stop) contains information about catch section of exception lifecycle.

ExceptionCatchStart event contains following initial attributes:
- EntryEIP - instruction pointer
- Method ID - the id of method
- Method Name - the name of method

ExceptionCatchStart and ExceptionCatchStop contains following dervied attributes:
- Managed thread ID
- Stack trace ID

## ExceptionFinally(Start/Stop)

Indicates the finally section of exception lifecycle.

The event ExceptionFinally(Start/Stop) contains following initial attributes:
- Entry EIP - instruction pointer
- Method ID
- Method Name

ExceptionFinally(Start/Stop) contain following derived attributes:
- Managed thread ID
- Stack trace ID

## ExceptionFilter(Start/Stop)

The event ExceptionFilter(Start/Stop) contains following initial attributes:
- Entry EIP - instruction pointer
- Method ID
- Method Name

ExceptionFilter(Start/Stop) contain following derived attributes:
- Managed thread ID
- Stack trace ID

# Array pool buffer events

Array pool events are fired from System.Private.CoreLib, so from managed code (ArrayPoolEventSource).
As all three kinds of events provide Pool IDs, Buckets IDs, Stack traces and managed threads IDs, it will be easy to link them

## Buffer allocated event

The event indicates the allocation of an array (buffer)

Initial Attributes:
- Buffer ID
- Pool ID
- Size of allocated array
- Reason why the buffer was allocated (Pooled = 0, OverMaximumSize = 1 (, Pool)
  - Pooled = 0, The pool is allocating the buffer when creating a new bucket
  - OverMaximumSize = 1, when we request too large buffer which was not yet allocated
  - PoolExhausted = 2, the pool has already allocated for pooling as many buffers of a particular size as it's allowed
- BucketId

Derived attributes:
- Managed thread ID
- Stack trace ID

## Buffer Rented Event

The event indicates giving the buffer to outer world

Initial attributes:
- Buffer ID
- Buffer size
- Pool ID
- Bucket ID

Derived events:
- Stacktrace ID
- Managed thread ID

## Buffer Returned event

The event when the buffer is returned to the pool

Initial attributes: 
- Buffer ID
- Buffer size
- Pool ID

Derived attributes:
- Managed Thread ID
- Stack Trace ID

## Buffer trimmed event

The event is raised when the buffer is attempted to be freed due to memory pressure or inactivity

The event contains following initial attributes:
- Pool ID
- Buffer ID
- Buffer size

Derived attributes:
- Managed Thread ID
- Stack Trace ID

## Buffer trim poll

The event reports that a check is being made to trim a buffer

Initial attributes:
- Milliseconds - milliseconds since the system has started 
- Pressure - current memory pressure, which is obtained form `GC` (`Low`, `Medium`, `High`)

Derived attributes:
- Managed Thread ID
- Stack Trace ID

# Assembly loader events

## AssemblyLoader(Start/Stop)

The AssemblyLoaderStart event is fired from two places: when loading the assembly from PE Image (`assemblynative.cpp` in `AssemblyNative::LoadFromPEImage`)
and when loading domain assembly, if it is cached (`assemblyspec.cpp` in `AssemblySpec::LoadDomainAssembly`), if the assembly is not cached,
we fallback to load from PE image. There is a cookie pattern in `AssemblyBindOperation`, which logs the start of assembly during construction
and logs the end of the assembly load during deconstruction.

The AssemblyLoaderStart event contains following initial attributes:
- Assembly name
- Assembly path
- Requesting assembly
- Assembly load context
- Requesting assembly load context

AssemblyLoader(Start/Stop) derived attributes:
- Managed thread id
- Stack trace id

The AssemblyLoaderStop events contains following initial attributes:
- Assembly name
- Assembly path
- Requesting assembly name
- Assembly load context
- Requesting assembly load context
- Success - if the loading was successful
- Result assembly name
- Result assembly path
- Cached - was the assembly cached

## AssemblyLoader/ResolutionAttempted

The `AssembyLoader/ResolutionAttempted` is called in `AssemblyBinderCommon::BindAssembly` (and in some other paths),
this event indicates the stage and the status of assembly loading.

The event contains following initial attributes:
- AssemblyName
- Stage
  - FindInLoadContext
  - AssemblyLoadContextLoad
  - ApplicationAssemblies
  - DefaultAssemblyLoadContextFallback
  - ResolveSatelliteAssembly
  - AssemblyLoadContextResolvingEvent
  - AppDomainAssemblyResolveEvent
- AssemblyLoadContext
- Result
  - Success
  - AssemblyNotFound
  - MismatchedAssemblyName
  - IncompatibleVersion
  - Failure
  - Exception
- ResultAssemblyName
- ResultAssemblyPath
- ErrorMessage

The event contains following dervied attributes:
- ManagedThreadId
- StacktraceId

## AssemblyLoader/KnownPathProbed

`AssemblyLoader/KnownPath` event is fired when a known path was probed from an assembly. The event is fired
from `AssemblyBinderCommon::BindByTpaList` method and some more places in `assemblybindercommon.cpp`.

The event contains following initial attributes:
- FilePath
- PathSource
  - ApplicationAssemblies
  - AppNativeImagePaths
  - AppPaths
  - PlatformResourceRoots
  - SatelliteSubdirectory
- Result (int)

The event contains following derived attributes:
- ManagedThreadId
- StacktraceId


## AssemblyLoader/AppDomainAssemblyResolveHandlerInvoked

The event is fired when handler for failed assembly resolve, after LoadContext handler has failed (`AppDomain.CurrentDomain.AssemblyResolve += (_, _) => null;`)

The events contains following initial attributes:
- AssemblyName
- HandlerName
- ResultAssemblyName
- ResultAssemblyPath

The event contains following derived attributes:
- ManagedThreadId
- StacktraceId

## AssemblyLoader/AssemblyLoadContextResolvingHandlerInvoked

The event is fired when handler in AssemblyLoadContext is failed.

The event contains following initial attributes:
- AssemblyName
- HandlerName
- AssemblyLoadContext
- ResultAssemblyName
- ResultAssemblyPath

The event contains following dervied attributes:
- ManagedThreadId
- StacktraceId

# Loader events

## Loader/AppDomain(Load/Unload/DCStop) events

The AppDomain(Load/Unload) event contains initial attributes:
- Application Domain ID
- Application domain flags (Default = 0x1, Executable = 0x2, Shared = 0, 0x4 - Application domain)
- Application domain name
- Application domain index

The managed thread id and stack trace id are -1.

## Loader/Assembly(Load/Unload)

The Assembly(Load/Unload) events contains following initial attributes:
- Assembly ID
- Application Domain ID
- Assembly flags (0x1 = domain neutral assembly, 0x2 = dynamic assembly, 0x4 = native image, 0x8 = collectible assembly)
- FQN
- Binding ID

Derived attributes:
- Managed thread ID
- Stack trace ID

## Loader/AssemblyDCStop

Enumerates existing assemblies, have the same attribute as `Loader/Assembly(Load/Unload)` events.

## Loader/Module(Load/Unload,DCStop)

The Module(Load/Unload) contains following initial attributes:
- Module ID
- Assembly ID
- Module flags (0x1 = neutral model, 0x2 = native image, 0x4 = dynamic module, 0x8 = manifest)
- Module path
- Module native path
- Managed pdb signature (GUID)
- Managed pdb age
- Managed pdb build path: Path to the location where the managed PDB that matches this module was built
- Native pdb signature
- Native pdb age
- Native pdb build path
- Module IL file name

Derived attributes:
- Managed thread ID
- Stack trace ID

## Loader/DomainModuleLoad or Loader/DomainModule(DC)Stop

Initial attributes:
- Module ID 
- Assembly ID
- AppDomain ID
- Module flags
- Module IL path
- Module native path

Derived attributes:
- Managed thread ID
- Stack trace ID

# Method events

## Method/InliningFailed

The event is raised in `jitinterface.cpp` in `CEEInfo::reportInliningDecision`. The `Method/InliningSuceeded` is also fired from this
method.

The event contains following initial attributes:
- Method namespace
- Method name
- Signature of the method
- Inliner namespace - the namespace of "parent" method
- Inliner name - the name of "parent" method
- Inliner signature
- Inlinee namespace - the namespace of method which we want to inline
- Inlinee name - the name of method which we want to inline
- Inlinee signature
- Fail always - whether the method is marked as not inlinable
- Fail reason - the reason, why inlining failed (subset of reasons, there are many of them)
  - "invalid argument number"
  - "invalid local number"
  - "compilation error"
  - "exceeds profit threshold"
  - "has exception handling"
  - "has endfilter"
  - "has endfinally"
  - "has leave"
  - "managed varargs"
  - "native varargs"
  - "has no body"
  - "has null pointer for ldelem"
  - "is array method"
  - "generic virtual"
  - "noinline per JitNoinline"
  - "noinline per IL/cached result"
  - "is synchronized"
  - "noinline per VM"
  - "no return opcode"
  - "ldfld needs helper"
  - "localloc size too large"
  - "rejected by log replay"
  - "skipped by complus request"
  - "maxstack too big"
  - "cannot get method info"
  - "unprofitable inline"
  - "random reject"
  - "uses stack crawl mark"
  - "stfld needs helper"
  - "too many arguments"
  - "too many locals"
  - "explicit tail prefix in callee"

## Method/InliningSucceeded

The event has following initial attributes:

- Method namespace
- Method name
- Signature of the method
- Inliner namespace - the namespace of "parent" method
- Inliner name - the name of "parent" method
- Inliner signature
- Inlinee namespace - the namespace of method which we want to inline
- Inlinee name - the name of method which we want to inline
- Inlinee signature

## Method/MemoryAllocatedForJitCode

The event is fired in `jitinterface.cpp` in `CEEJitInfo::allocMem`.

The event has following initial attributes:
- Method ID
- Module ID
- Hot code request size
- RO Data request size
- Allocated size for code
- Allocation flags
  - CORJIT_ALLOCMEM_DEFAULT_CODE_ALIGN (0x00000000, The code will use the normal alignment) 
  - CORJIT_ALLOCMEM_FLG_16BYTE_ALIGN (0x00000001, The code will be 16-byte aligned)
  - CORJIT_ALLOCMEM_FLG_RODATA_16BYTE_ALIGN (0x00000002, The read-only data will be 16-byte aligned) 
  - CORJIT_ALLOCMEM_FLG_32BYTE_ALIGN (0x00000004, The code will be 32-byte aligned)
  - CORJIT_ALLOCMEM_FLG_RODATA_32BYTE_ALIGN (0x00000008, The read-only data will be 32-byte aligned)

## Method/JittingStarted

Raised when the method is JIT-compliled. The event is raised in `jitinterface.cpp` in `CompileMethodWithEtwWrapper` method.

The event contains following attributes:
- Method ID
- Module ID
- Method Token
- Method IL size
- Method namespace
- Method name 
- Method signature

## Method/(Load/Unload)Verbose

The event contains following initial attributes:
- Method ID
- Module ID
- Method start address
- Method size (0 for dynamic methods and JIT helpers)
- Method token
- Method flags (0x1 - Dynamic, 0x2 - Generic methods, 0x4 - jit-compiled method, 0x8 helper-method)
- Method namespace
- Method name
- Method signature
- ReJIT ID - ReJIT ID of the method.
- Optimization Tier = (Unknown, MinOptJitted, Optimized, QuickJitted, OptimizedTier1, OptimizedTier1OSR, R2R, PreJIT)

## Method/R2RGetEntryPointStart

Raised when an R2R entry point lookup starts. The event is raised in `readytoruninfo.cpp` in method `ReadyToRunInfo::GetEntryPoint`.

The event has the following initial attributes:
- Method ID

The event contains following derived attributes:
- Managed thread ID
- Stack trace ID

## Method/R2RGetEntryPoint

Raised when an R2R entry point lookup ends. If the R2R is disabled or we failed to find entry point we still will get this event,
but without `R2RGetEntryPointStart`. This event is also raised in `readytoruninfo.cpp` in `ReadyToRunInfo::GetEntryPoint`.

The event contains following initial attributes:
- Method ID
- Method namespace
- Method name
- Method signature
- Entry Point 

The event contains following derived attributes:
- Managed thread ID
- Stack trace ID

## Method/TailCallSucceeded

Raised by the JIT compiler when a method can be successfully tail called.

The event contains following initial attributes:
- Method (which is compiled) namespace
- Method (which is compiled) name
- Method (which is compiled) signature
- Caller namespace
- Caller name
- Caller signature
- Callee namespace
- Callee name
- Tail prefix (true/false)
- Tail call type (OptimizedTailCall, RecursiveLoop, HelperAssistedTailCall)

## Method/TailCallFailed

Raised when JIT failed to tail-call method

The event contains following initial attributes:
- Method (which is compiled) namespace
- Method (which is compiled) name
- Method (which is compiled) signature
- Caller namespace
- Caller name
- Caller signature
- Callee namespace
- Callee name
- Tail prefix (true/false)
- Fail reason

# Tasks/Thread/ThreadPool events

The task events are fired from managed code, from System.Private.CoreLib, in System.Threading.Tasks namespace.

## TaskExecute/Start 

The event is fired before the task starts.

The event contains following initial attribute:
- Original task scheduler ID - the id of a scheduler, if we have a current (`t_currentTask`, The currently executing task) task (the current task is stored in TLS (thread local storage, `ThreadStatic` attribute)),
  then we will use scheduler from the current task, if we don't have current task, we will use `TaskScheduler.Current`.
- Task ID
- Previous task ID

The event contains following derived attributes:
- Managed thread ID
- Stack trace ID

## TaskExecute/Stop

The event is fired after the task is ended (`Task::ExecuteWithThreadLocal`)

The event contains following initial attributes:
- Original task scheduler ID (same as in `TaskExecute/Start`)
- Task ID
- Previous task ID
- Is exceptional (do task have an exception)

The event contains following derived attributes:
- Managed thread ID
- Stack trace ID

## TaskWait/Send

The event is fired when we do wait in `Task::InternalWaitCore` and in `TaskAwaiter` in OnCompleted methods, which queue callbacks after completion.

The event contains following initial attributes:
- Original task scheduler ID (same as in `TaskExecute/Start`)
- Original task ID
- Task ID
- Task wait kind (Synchronous = Task.Wait(), sync call, Asynchronous = await))
- Continue with task ID (0 in case of sync wait)

The event contains following derived attributes:
- Managed thread ID
- Stack trace ID

## TaskWait/Stop

The event is fired when we do wait in `Task::InternalWaitCore` and in `TaskAwaiter` in OnCompleted methods,
if we enable CLR events logging, then the continuation will be wrapped with additional logging, to fire the event.

The initial attributes:
- Original task scheduler ID (same as in `TaskExecute/Start`)
- Original task ID
- Task ID

The derived attributes:
- Managed thread ID
- Stack trace ID

## TaskScheduled/Send

The event is fired from `TaskScheduler`, when the task is queued or when it is executed inlined.

The initial attributes:
- Original task scheduler ID (same as in `TaskExecute/Start`)
- Original task ID
- Task ID
- Creating task ID
- Task creation options
  - 0x0 - None 
  - 0x01 - PreferFairness 
  - 0x02 - LongRunning
  - 0x04 - AttachedToParent 
  - 0x08 - DenyChildAttach
  - 0x10 - Hide scheduler
  - 0x40 - RunContinuationsAsynchronously

The dervied attributes:
- Managed thread ID
- Stack trace ID

## TaskWaitContinuationStarted

The event is fired before invoking the continuation, there is only one usage of this event is in `YieldAwaiter`, which is
used in `Task.Yield()`

The event contains following initial attributes:
- Task ID

The event contains following derived attributes:
- Managed thread ID
- Stacktrace ID

## TaskWaitContinuationComplete

The event is fired after wait ends, from `Task::InternalWaitCore` and from `TaskAwaiter` (when creating a wrapper function for continuation)

The initial attributes:
- Task id

The derived attributes:
- Manged thread ID
- Stacktrace ID

## AwaitTaskContinuationScheduled

The event is fired when the an asynchronous continuation for a task is scheduled.
The event is fired in awaiters, in ContinueWith.

The event has following initial attributes:
- Original task scheduler ID (same as in `TaskExecute/Start`)
- Original task ID
- Continue with ID

Derived attributes:
- Managed thread ID
- Stack trace ID

## IncompleteAsyncMethod

Event which indicates that something went wrong.

`If the state machine is being finalized, something went wrong during its processing,
e.g. it awaited something that got collected without itself having been completed.
Fire an event with details about the state machine to help with debugging.`

The event has following initial attributes:
- Async state machine state = the description which shows the current state

## ThreadPoolWorkerThread(Start/Stop)

Indicates that worker thread starts or stopped

Contains following initial attributes:
- Active worker threads count
- Retired worker threads count

## ThreadPoolWorkerThreadRetirement(Start/Stop)

Start = worker thread retires, stop = worker thread active again

Contains following initial attributes:
- Active worker threads count
- Retired worker threads count

## ThreadPoolWorkerThreadAdjustmentSample

A measurement of throughput with a certain concurrency level, in an instant of time.

Initial attributes:
- Throughput = number of completions per unit of time

## ThreadPoolWorkerThreadAdjustmentAdjustment

Records a change in control, when the thread injection (hill-climbing) algorithm determines that a change in concurrency level is in place.

Initial attributes:
- Average throughput
- New number of active worker threads
- Reason:
  - Warmup
  - Initializing
  - Random move
  - Climbing move
  - Change point
  - Stabilizing
  - Starvation
  - Thread timed out

## IOThread Create/Terminate

Fired when the IO thread is created/terminated

Initial attributes:
- Number of IO threads
- Number of retired IO threads

## IOThread Retire/Unretire

When an IO thread becomes a retirement candidate
or
When n IO thread is unretired because of I/O that arrives within a waiting period after the thread becomes a retirement candidate.

Initial attributes:
- Number of IO threads
- Number of retired IO threads

## ThreadPoolDequeueWork

The event is fired when the work item is dequeued in the thread pool. The Dequeue event is fired from `ThreadPoolWorkQueue::Dispatch`
method, which is responsible for getting work for worker thread.

The event has following initial attributes:
- WorkID - which is the hash of dequeued work item

The event has following derived attributes:
- ManagedThreadId
- StacktraceId

## ThreadPoolEnqueueWork

The event is fired when the work item is queued in the thread pool. The queue event is fired from `ThreadPoolWorkingQueue::Enqueue` and
`ThreadPoolWorkingQueue::EnqueueAtHighPriority` methods.

The event contains following initial attributes:
- WorkID - as in dequeue event it the hash of work item.

The event contains following derived attributes:
- ManagedThreadId
- StacktraceId

## ThreadPoolWorkerThread/Wait

The worker thread can wait to enter the loop where it tries to get work from queues. To enter this loop the thread waits on semaphore, 
the semaphore is designed to control how many worker threads can do work. When the wait starts the `ThreadPoolWorkerThreadWait` is fired.

The event has following initial attributes:
- ActiveWorkerThreadCount
- RetiredWorkerThreadCount

The event contains following dervied attributes:
- ManagedThreadId
- StacktraceId

# Native events

## AppDomainResourceManagement/ThreadCreated

Called when the thread is created.

The event has following initial attributes:
- Managed thread ID
- App domain ID
- Flags
- Managed thread index
- OS Thread ID

# HTTP events 

Http events are fired through `HttpTelemetry` class in managed code (`System.Net.Http`)

## Request(Start/Stop)

RequestStart initial attributes:
 - Scheme
 - Host
 - Port
 - Path and query
 - Major version
 - Minor version
 - Version policy (RequestVersionOrLower, RequestVersionOrHigher, RequestVersionExact)

Derived attributes:
- Managed thread ID
- Stacktrace ID

RequestStop contains just an event ID

## RequestFailed

Just an event with ID as RequestStop

## Connection(Established/Closed)

Fired when the connection is established or closed.

Initial attributes:
- Major version
- Minor version

Derived attributes:
- Managed thread ID
- Stacktrace ID

## RequestLeftQueue

Fired when HTTP request leaves the request queue

Initial attributes:
- Time which was spend in queue in milliseconds
- Version major
- Version minor

Derived attributes:
- Managed thread ID
- Stacktrace ID

## RequestHeaders(Start/Stop)

Fired when the request for headers starts/stops.

Contains only:
- Managed thread ID
- Stacktrace ID

## RequestContent(Start/Stop)

Fired when starting/stopping sending request content

RequestContent(Start/Stop) contains:
- Managed thread ID
- Stacktrace ID

RequestContentStop contains:
- The number of bytes which was sent

## ResponseHeaders(Start/Stop)

Http response for headers started or stopped

ResponseHeaders(Start/Stop) contains:
- Managed thread ID
- Stacktrace ID

## ResponseContent(Start/Stop)

Http response for content started/stopped

ResponseContent(Start/Stop) contains just:
- Managed thread ID
- Stacktrace ID

# Sockets

The events is fired from `System.Net.Sockets` library, the `SocketsTelemetry` class is responsible for firing events.

## Connect Start

The even is fired from `Socket` class, from `DoConnect` and `ConnectAsync` methods. 
The event has following initial attributes attributes:
- Address

The event has following derived attributes:
- Managed thread ID
- Stacktrace ID

## Connect Stop

The event is mainly fired from `Socket` class, from `DoConnect` and `ConnectAsync` methods.

The event contains following derived attributes:
- Managed Thread ID
- Stacktrace ID

## Connect Failed

The event is fired from the same places as `Connect Stop`.

The event has following initial attributes:
- Error code
- Exception message

The event has following derived attributes:
- Managed thread ID
- Stacktrace ID

## Accept Start

The event is fired from `Socket` class from `Accept` and `AcceptAsync` methods.

The event has following initial attributes:
- Address string

The event has following derived attributes:
- Managed thread ID
- Stacktrace ID

## Accept Stop

The event is mainly fired from `Socket` class from `Accept` and `AcceptAsync` methods.

The event has following derived attributes:
- Manage thread ID
- Stacktrace ID

## Accept Failed

The event is fired from the same places as from `Accept Stop`.

The event has following initial attributes:
- Error code
- Exception message

The event has following derived attributes:
- Managed thread ID
- Stacktrace ID