#include "ProcfilerCorProfilerCallback.h"
#include <performance_counter.h>
#include <env_constants.h>

#include "clr/profilerstring.hpp"
#include "shadowstack/serializers/BinaryShadowStackSerializer.h"
#include "shadowstack/serializers/DebugShadowStackSerializer.h"
#include "shadowstack/serializers/EventPipeShadowStackSerializer.h"
#include "sigparser/sigparser.h"
#include "sigparser/sigparserimpl.hpp"

ProcfilerCorProfilerCallback* ourCallback;

ProcfilerCorProfilerCallback* GetCallbackInstance() {
    return ourCallback;
}

void StaticHandleFunctionEnter(const FunctionID functionId) {
    GetCallbackInstance()->HandleFunctionEnter(functionId);
}

void StaticHandleFunctionLeave(const FunctionID functionId) {
    GetCallbackInstance()->HandleFunctionLeave(functionId);
}

void StaticHandleFunctionTailCall(const FunctionID functionId) {
    GetCallbackInstance()->HandleFunctionTailCall(functionId);
}

void StaticHandleFunctionEnter2(FunctionID funcId, UINT_PTR clientData, COR_PRF_FRAME_INFO func,
                                COR_PRF_FUNCTION_ARGUMENT_INFO* argumentInfo) {
    GetCallbackInstance()->HandleFunctionEnter2(funcId, argumentInfo);
}

void StaticHandleFunctionLeave2(FunctionID funcId, UINT_PTR clientData, COR_PRF_FRAME_INFO func,
                                COR_PRF_FUNCTION_ARGUMENT_RANGE* retvalRange) {
    GetCallbackInstance()->HandleFunctionLeave2(funcId);
}

void StaticHandleFunctionTailCall2(FunctionID funcId, UINT_PTR clientData, COR_PRF_FRAME_INFO func) {
    GetCallbackInstance()->HandleFunctionTailCall2(funcId);
}

int64_t ProcfilerCorProfilerCallback::GetCurrentTimestamp() const {
    LARGE_INTEGER value;

    if (!QueryPerformanceCounter2(&value)) {
        myLogger->LogInformation("Failed to get current timestamp");
        return -1;
    }

    return value.QuadPart;
}

void ProcfilerCorProfilerCallback::HandleFunctionEnter(const FunctionID funcId) const {
    const auto timestamp = GetCurrentTimestamp();
    myShadowStack->AddFunctionEnter(funcId, GetCurrentManagedThreadId(), timestamp);
}

void ProcfilerCorProfilerCallback::HandleFunctionLeave(const FunctionID funcId) const {
    const auto timestamp = GetCurrentTimestamp();
    myShadowStack->AddFunctionFinished(funcId, GetCurrentManagedThreadId(), timestamp);
}

void ProcfilerCorProfilerCallback::HandleFunctionTailCall(const FunctionID funcId) const {
    const auto timestamp = GetCurrentTimestamp();
    myShadowStack->AddFunctionFinished(funcId, GetCurrentManagedThreadId(), timestamp);
}

void ProcfilerCorProfilerCallback::HandleFunctionEnter2(FunctionID funcId,
                                                        COR_PRF_FUNCTION_ARGUMENT_INFO* argumentInfo) const {
    ObjectID id;
    myObjectsManager->TryGetThisObjectId(funcId, argumentInfo, &id);

    HandleFunctionEnter(funcId);
}

void ProcfilerCorProfilerCallback::HandleFunctionLeave2(const FunctionID funcId) const {
    HandleFunctionLeave(funcId);
}

void ProcfilerCorProfilerCallback::HandleFunctionTailCall2(const FunctionID funcId) const {
    HandleFunctionTailCall(funcId);
}

ICorProfilerInfo15* ProcfilerCorProfilerCallback::GetProfilerInfo() const {
    return myProfilerInfo;
}

HRESULT ProcfilerCorProfilerCallback::Initialize(IUnknown* pICorProfilerInfoUnk) {
    myLogger->LogInformation("Started initializing CorProfiler callback");
    const auto ptr = reinterpret_cast<void**>(&this->myProfilerInfo);

    HRESULT result = pICorProfilerInfoUnk->QueryInterface(IID_ICorProfilerInfo15, ptr);
    if (FAILED(result)) {
        myLogger->LogInformation("Failed to query interface: " + std::to_string(result));
        return E_FAIL;
    }

    InitializeShadowStack();
    myObjectsManager = new ObjectsManager(myProfilerInfo);

    const auto produceObjectBinStacks = IsEnvVarTrue(produceObjectBinStacksEnv);

    DWORD eventMask = COR_PRF_MONITOR_ENTERLEAVE | COR_PRF_MONITOR_EXCEPTIONS;
    if (produceObjectBinStacks) {
        eventMask |= COR_PRF_ENABLE_FUNCTION_ARGS;
    }

    result = myProfilerInfo->SetEventMask(eventMask);
    if (FAILED(result)) {
        myLogger->LogInformation("Failed to set event mask: " + std::to_string(result));
        return E_FAIL;
    }

    if (produceObjectBinStacks) {
        result = myProfilerInfo->SetEnterLeaveFunctionHooks2(
            StaticHandleFunctionEnter2,
            StaticHandleFunctionLeave2,
            StaticHandleFunctionTailCall2
        );
    } else {
        result = myProfilerInfo->SetEnterLeaveFunctionHooks(
            StaticHandleFunctionEnter,
            StaticHandleFunctionLeave,
            StaticHandleFunctionTailCall
        );
    }

    if (FAILED(result)) {
        myLogger->LogInformation("Failed to set enter-leave hooks: " + std::to_string(result));
        return E_FAIL;
    }

    myLogger->LogInformation("Initialized CorProfiler callback");
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::MovedReferences(ULONG cMovedObjectIDRanges,
                                                      ObjectID* oldObjectIDRangeStart,
                                                      ObjectID* newObjectIDRangeStart,
                                                      ULONG* cObjectIDRangeLength) {
    myObjectsManager->HandleObjectsMove(cMovedObjectIDRanges,
                                        oldObjectIDRangeStart,
                                        newObjectIDRangeStart,
                                        cObjectIDRangeLength);

    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ObjectAllocated(ObjectID objectId, ClassID classId) {
    myObjectsManager->HandleObjectAllocation(objectId);
    return S_OK;
}

void ProcfilerCorProfilerCallback::InitializeShadowStack() {
    const auto onlineSerialization = IsEnvVarTrue(onlineSerializationEnv);
    myShadowStack = new ShadowStack(this->myProfilerInfo, myLogger, onlineSerialization);

    if (onlineSerialization) {
        myShadowStackSerializer = new ShadowStackSerializerStub();
    } else {
        if (IsEnvVarDefined(binaryStackSavePathEnv)) {
            myShadowStackSerializer = new BinaryShadowStackSerializer(myProfilerInfo, myLogger);
        } else if (IsEnvVarDefined(shadowStackDebugSavePathEnv)) {
            myShadowStackSerializer = new DebugShadowStackSerializer(myProfilerInfo, myLogger);
        } else if (IsEnvVarDefined(eventPipeSaveShadowStackEnv)) {
            myShadowStackSerializer = new EventPipeShadowStackSerializer(myProfilerInfo, myLogger);
        } else {
            myShadowStackSerializer = new ShadowStackSerializerStub();
        }
    }

    myShadowStackSerializer->Init();
}

HRESULT ProcfilerCorProfilerCallback::ExceptionCatcherEnter(const FunctionID functionId, ObjectID objectId) {
    myShadowStack->HandleExceptionCatchEnter(functionId, GetCurrentManagedThreadId(), GetCurrentTimestamp());
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::Shutdown() {
    myLogger->LogInformation("Shutting down profiler");

    myShadowStack->SuppressFurtherMethodsEvents();
    myLogger->LogInformation("Suppressed further events");

    myShadowStack->WaitForPendingMethodsEvents();
    myLogger->LogInformation("Finished wait for events");

    myShadowStack->AdjustShadowStacks();
    myLogger->LogInformation("Adjusted shadow-stacks");

    myShadowStackSerializer->Serialize(myShadowStack);
    myLogger->LogInformation("Serialized shadow-stack");

    delete myShadowStack;
    myLogger->LogInformation("Deleted shadow-stack");

    if (myProfilerInfo != nullptr) {
        myProfilerInfo->Release();
        myProfilerInfo = nullptr;
    }

    myLogger->LogInformation("Shutted down profiler");
    return S_OK;
}

ProcfilerCorProfilerCallback::ProcfilerCorProfilerCallback(ProcfilerLogger* logger) : myRefCount(0),
    myProfilerInfo(nullptr), myLogger(logger) {
    ourCallback = this;
    myShadowStack = nullptr;
    myShadowStackSerializer = nullptr;
    myObjectsManager = nullptr;
}

HRESULT ProcfilerCorProfilerCallback::AppDomainCreationStarted(AppDomainID appDomainId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::AppDomainCreationFinished(AppDomainID appDomainId, HRESULT hrStatus) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::AppDomainShutdownStarted(AppDomainID appDomainId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::AppDomainShutdownFinished(AppDomainID appDomainId, HRESULT hrStatus) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::AssemblyLoadStarted(AssemblyID assemblyId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::AssemblyLoadFinished(AssemblyID assemblyId, HRESULT hrStatus) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::AssemblyUnloadStarted(AssemblyID assemblyId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::AssemblyUnloadFinished(AssemblyID assemblyId, HRESULT hrStatus) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ModuleLoadStarted(ModuleID moduleId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ModuleLoadFinished(ModuleID moduleId, HRESULT hrStatus) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ModuleUnloadStarted(ModuleID moduleId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ModuleUnloadFinished(ModuleID moduleId, HRESULT hrStatus) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ModuleAttachedToAssembly(ModuleID moduleId, AssemblyID AssemblyId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ClassLoadStarted(ClassID classId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ClassLoadFinished(ClassID classId, HRESULT hrStatus) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ClassUnloadStarted(ClassID classId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ClassUnloadFinished(ClassID classId, HRESULT hrStatus) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::FunctionUnloadStarted(FunctionID functionId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::JITCompilationStarted(FunctionID functionId, BOOL fIsSafeToBlock) {
    return S_OK;
}

HRESULT
ProcfilerCorProfilerCallback::JITCompilationFinished(FunctionID functionId, HRESULT hrStatus, BOOL fIsSafeToBlock) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::JITCachedFunctionSearchStarted(FunctionID functionId, BOOL* pbUseCachedFunction) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::JITCachedFunctionSearchFinished(FunctionID functionId, COR_PRF_JIT_CACHE result) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::JITFunctionPitched(FunctionID functionId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::JITInlining(FunctionID callerId, FunctionID calleeId, BOOL* pfShouldInline) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ThreadCreated(ThreadID threadId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ThreadDestroyed(ThreadID threadId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ThreadAssignedToOSThread(ThreadID managedThreadId, DWORD osThreadId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RemotingClientInvocationStarted() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RemotingClientSendingMessage(GUID* pCookie, BOOL fIsAsync) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RemotingClientReceivingReply(GUID* pCookie, BOOL fIsAsync) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RemotingClientInvocationFinished() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RemotingServerReceivingMessage(GUID* pCookie, BOOL fIsAsync) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RemotingServerInvocationStarted() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RemotingServerInvocationReturned() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RemotingServerSendingReply(GUID* pCookie, BOOL fIsAsync) {
    return S_OK;
}

HRESULT
ProcfilerCorProfilerCallback::UnmanagedToManagedTransition(FunctionID functionId, COR_PRF_TRANSITION_REASON reason) {
    return S_OK;
}

HRESULT
ProcfilerCorProfilerCallback::ManagedToUnmanagedTransition(FunctionID functionId, COR_PRF_TRANSITION_REASON reason) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RuntimeSuspendStarted(COR_PRF_SUSPEND_REASON suspendReason) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RuntimeSuspendFinished() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RuntimeSuspendAborted() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RuntimeResumeStarted() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RuntimeResumeFinished() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RuntimeThreadSuspended(ThreadID threadId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RuntimeThreadResumed(ThreadID threadId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ObjectsAllocatedByClass(ULONG cClassCount, ClassID* classIds, ULONG* cObjects) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ObjectReferences(ObjectID objectId,
                                                       ClassID classId,
                                                       ULONG cObjectRefs,
                                                       ObjectID* objectRefIds) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::RootReferences(ULONG cRootRefs, ObjectID* rootRefIds) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionThrown(ObjectID thrownObjectId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionSearchFunctionEnter(FunctionID functionId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionSearchFunctionLeave() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionSearchFilterEnter(FunctionID functionId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionSearchFilterLeave() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionSearchCatcherFound(FunctionID functionId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionOSHandlerEnter(UINT_PTR) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionOSHandlerLeave(UINT_PTR) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionUnwindFunctionEnter(FunctionID functionId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionUnwindFunctionLeave() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionUnwindFinallyEnter(FunctionID functionId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionUnwindFinallyLeave() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionCatcherLeave() {
    return S_OK;
}

HRESULT
ProcfilerCorProfilerCallback::COMClassicVTableCreated(ClassID wrappedClassId,
                                                      const GUID& implementedIID,
                                                      void* pVTable,
                                                      ULONG cSlots) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::COMClassicVTableDestroyed(ClassID wrappedClassId,
                                                                const GUID& implementedIID,
                                                                void* pVTable) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionCLRCatcherFound() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ExceptionCLRCatcherExecute() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ThreadNameChanged(ThreadID threadId, ULONG cchName, WCHAR* name) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::GarbageCollectionStarted(int cGenerations,
                                                               BOOL* generationCollected,
                                                               COR_PRF_GC_REASON reason) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::SurvivingReferences(ULONG cSurvivingObjectIDRanges,
                                                          ObjectID* objectIDRangeStart,
                                                          ULONG* cObjectIDRangeLength) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::GarbageCollectionFinished() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::FinalizeableObjectQueued(DWORD finalizerFlags, ObjectID objectID) {
    return S_OK;
}

HRESULT
ProcfilerCorProfilerCallback::RootReferences2(ULONG cRootRefs,
                                              ObjectID* rootRefIds,
                                              COR_PRF_GC_ROOT_KIND* rootKinds,
                                              COR_PRF_GC_ROOT_FLAGS* rootFlags,
                                              UINT_PTR* rootIds) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::HandleCreated(GCHandleID handleId, ObjectID initialObjectId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::HandleDestroyed(GCHandleID handleId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::InitializeForAttach(IUnknown* pCorProfilerInfoUnk,
                                                          void* pvClientData,
                                                          UINT cbClientData) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ProfilerAttachComplete() {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ProfilerDetachSucceeded() {
    return S_OK;
}

HRESULT
ProcfilerCorProfilerCallback::ReJITCompilationStarted(FunctionID functionId, ReJITID rejitId, BOOL fIsSafeToBlock) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::GetReJITParameters(ModuleID moduleId,
                                                         mdMethodDef methodId,
                                                         ICorProfilerFunctionControl* pFunctionControl) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ReJITCompilationFinished(FunctionID functionId,
                                                               ReJITID rejitId,
                                                               HRESULT hrStatus,
                                                               BOOL fIsSafeToBlock) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ReJITError(ModuleID moduleId,
                                                 mdMethodDef methodId,
                                                 FunctionID functionId,
                                                 HRESULT hrStatus) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::MovedReferences2(ULONG cMovedObjectIDRanges,
                                                       ObjectID* oldObjectIDRangeStart,
                                                       ObjectID* newObjectIDRangeStart,
                                                       SIZE_T* cObjectIDRangeLength) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::SurvivingReferences2(ULONG cSurvivingObjectIDRanges,
                                                           ObjectID* objectIDRangeStart,
                                                           SIZE_T* cObjectIDRangeLength) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ConditionalWeakTableElementReferences(ULONG cRootRefs,
                                                                            ObjectID* keyRefIds,
                                                                            ObjectID* valueRefIds,
                                                                            GCHandleID* rootIds) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::GetAssemblyReferences(const WCHAR* wszAssemblyPath,
                                                            ICorProfilerAssemblyReferenceProvider* pAsmRefProvider) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::ModuleInMemorySymbolsUpdated(ModuleID moduleId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::DynamicMethodJITCompilationStarted(FunctionID functionId, BOOL fIsSafeToBlock,
                                                                         LPCBYTE ilHeader, ULONG cbILHeader) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::DynamicMethodJITCompilationFinished(FunctionID functionId, HRESULT hrStatus,
                                                                          BOOL fIsSafeToBlock) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::DynamicMethodUnloaded(FunctionID functionId) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::EventPipeProviderCreated(EVENTPIPE_PROVIDER provider) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::LoadAsNotificationOnly(BOOL* pbNotificationOnly) {
    return S_OK;
}

HRESULT ProcfilerCorProfilerCallback::EventPipeEventDelivered(EVENTPIPE_PROVIDER provider,
                                                              DWORD eventId,
                                                              DWORD eventVersion,
                                                              ULONG cbMetadataBlob,
                                                              LPCBYTE metadataBlob,
                                                              ULONG cbEventData,
                                                              LPCBYTE eventData,
                                                              LPCGUID pActivityId,
                                                              LPCGUID pRelatedActivityId,
                                                              ThreadID eventThread,
                                                              ULONG numStackFrames,
                                                              UINT_PTR* stackFrames) {
    return S_OK;
}

ProcfilerCorProfilerCallback::~ProcfilerCorProfilerCallback() {
    myProfilerInfo = nullptr;

    delete myLogger;
    myLogger = nullptr;

    delete myShadowStack;
    myShadowStack = nullptr;

    delete myShadowStackSerializer;
    myShadowStackSerializer = nullptr;

    delete myObjectsManager;
    myObjectsManager = nullptr;
}

DWORD ProcfilerCorProfilerCallback::GetCurrentManagedThreadId() const {
    ThreadID threadId;
    myProfilerInfo->GetCurrentThreadID(&threadId);

    DWORD id;
    myProfilerInfo->GetThreadInfo(threadId, &id);

    return id;
}


HRESULT STDMETHODCALLTYPE ProcfilerCorProfilerCallback::QueryInterface(REFIID riid, void** ppvObject) {
    if (riid == IID_ICorProfilerCallback11 ||
        riid == IID_ICorProfilerCallback10 ||
        riid == IID_ICorProfilerCallback9 ||
        riid == IID_ICorProfilerCallback8 ||
        riid == IID_ICorProfilerCallback7 ||
        riid == IID_ICorProfilerCallback6 ||
        riid == IID_ICorProfilerCallback5 ||
        riid == IID_ICorProfilerCallback4 ||
        riid == IID_ICorProfilerCallback3 ||
        riid == IID_ICorProfilerCallback2 ||
        riid == IID_ICorProfilerCallback ||
        riid == IID_IUnknown) {
        *ppvObject = this;
        this->AddRef();
        return S_OK;
    }

    *ppvObject = nullptr;
    return E_NOINTERFACE;
}

ULONG STDMETHODCALLTYPE ProcfilerCorProfilerCallback::AddRef() {
    return std::atomic_fetch_add(&this->myRefCount, 1) + 1;
}

ULONG STDMETHODCALLTYPE ProcfilerCorProfilerCallback::Release() {
    const int count = std::atomic_fetch_sub(&this->myRefCount, 1) - 1;

    if (count <= 0) {
        delete this;
    }

    return count;
}
