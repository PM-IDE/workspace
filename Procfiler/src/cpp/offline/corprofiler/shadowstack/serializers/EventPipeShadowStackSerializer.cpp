#include "EventPipeShadowStackSerializer.h"

EventPipeShadowStackSerializer::EventPipeShadowStackSerializer(ICorProfilerInfo12* profilerInfo,
                                                               ProcfilerLogger* logger) {
    myLogger = logger;
    myProfilerInfo = profilerInfo;
}

void EventPipeShadowStackSerializer::Init() {
    InitializeProvidersAndEvents();
}

void EventPipeShadowStackSerializer::Serialize(ShadowStack* shadowStack) {
    HRESULT hr;

    auto events = shadowStack->GetAllStacks();
    std::map<FunctionID, FunctionInfo> resolvedFunctions;

    for (const auto& pair: *events) {
        auto offlineEvents = dynamic_cast<EventsWithThreadIdOffline*>(pair.second);
        if (offlineEvents == nullptr) continue;

        auto threadId = pair.first;

        for (const auto& event: *(offlineEvents->Events)) {
            hr = FAILED(LogFunctionEvent(event, threadId, resolvedFunctions));
            if (hr) {
                myLogger->LogError("Failed to send a method start or end event, error: " + std::to_string(hr));
            }
        }
    }

    myLogger->LogInformation("Logged method start and end events to event pipe");

    for (const auto& pair: resolvedFunctions) {
        hr = FAILED(LogMethodInfo(pair.first, pair.second));
        if (hr) {
            myLogger->LogError("Failed to send a method info event, error: " + std::to_string(hr));
        }
    }

    myLogger->LogInformation("Logged all method info events");
}

HRESULT EventPipeShadowStackSerializer::DefineProcfilerMethodStartEvent() {
    return DefineMethodStartOrEndEventInternal(ToWString("ProcfilerMethodStart"),
                                               myEventPipeProvider,
                                               &myMethodStartEvent,
                                               myProfilerInfo,
                                               ourMethodStartEventId);
}

HRESULT EventPipeShadowStackSerializer::DefineProcfilerMethodEndEvent() {
    return DefineMethodStartOrEndEventInternal(ToWString("ProcfilerMethodEnd"),
                                               myEventPipeProvider,
                                               &myMethodEndEvent,
                                               myProfilerInfo,
                                               ourMethodEndEventId);
}

HRESULT EventPipeShadowStackSerializer::DefineProcfilerMethodInfoEvent() {
    COR_PRF_EVENTPIPE_PARAM_DESC eventParameters[] = {
            {COR_PRF_EVENTPIPE_UINT64, 0, ToWString("FunctionId").c_str()},
            {COR_PRF_EVENTPIPE_STRING, 0, ToWString("FunctionName").c_str()},
    };

    auto paramsCount = sizeof(eventParameters) / sizeof(COR_PRF_EVENTPIPE_PARAM_DESC);

    return myProfilerInfo->EventPipeDefineEvent(
            myEventPipeProvider,
            ourMethodInfoEventName.c_str(),
            ourMethodInfoEventId,
            0,
            1,
            COR_PRF_EVENTPIPE_LOGALWAYS,
            0,
            false,
            paramsCount,
            eventParameters,
            &myMethodInfoEvent
    );
}

HRESULT EventPipeShadowStackSerializer::DefineProcfilerEventPipeProvider() {
    return myProfilerInfo->EventPipeCreateProvider(ourEventPipeProviderName.c_str(), &myEventPipeProvider);
}

HRESULT EventPipeShadowStackSerializer::DefineMethodStartOrEndEventInternal(const wstring& eventName,
                                                                            EVENTPIPE_PROVIDER provider,
                                                                            EVENTPIPE_EVENT* outEventId,
                                                                            ICorProfilerInfo12* profilerInfo,
                                                                            UINT32 eventId) {
    COR_PRF_EVENTPIPE_PARAM_DESC eventParameters[] = {
            {COR_PRF_EVENTPIPE_UINT64, 0, ToWString("Timestamp").c_str()},
            {COR_PRF_EVENTPIPE_UINT64, 0, ToWString("FunctionId").c_str()},
            {COR_PRF_EVENTPIPE_UINT64, 0, ToWString("ThreadId").c_str()},
    };

    auto paramsCount = sizeof(eventParameters) / sizeof(COR_PRF_EVENTPIPE_PARAM_DESC);

    return profilerInfo->EventPipeDefineEvent(
            provider,
            eventName.c_str(),
            eventId,
            0,
            1,
            COR_PRF_EVENTPIPE_LOGALWAYS,
            0,
            false,
            paramsCount,
            eventParameters,
            outEventId
    );
}

HRESULT EventPipeShadowStackSerializer::InitializeProvidersAndEvents() {
    myLogger->LogInformation("Starting writing shadow stack to event pipe");

    HRESULT hr;
    if ((hr = DefineProcfilerEventPipeProvider()) != S_OK) {
        auto logMessage = "Failed to initialize Event Pipe Provider, HR = " + std::to_string(hr);
        myLogger->LogError(logMessage);
        return hr;
    }

    if ((hr = DefineProcfilerMethodStartEvent()) != S_OK) {
        auto logMessage = "Failed to initialize method start event, HR = " + std::to_string(hr);
        myLogger->LogError(logMessage);
        return hr;
    }

    if ((hr = DefineProcfilerMethodEndEvent()) != S_OK) {
        auto logMessage = "Failed to initialize method end event, HR = " + std::to_string(hr);
        myLogger->LogError(logMessage);
        return hr;
    }

    if ((hr = DefineProcfilerMethodInfoEvent()) != S_OK) {
        auto logMessage = "Failed to initialize method info event, HR = " + std::to_string(hr);
        myLogger->LogError(logMessage);
        return hr;
    }

    myLogger->LogInformation("Initialized provider and all needed events");

    return S_OK;
}

HRESULT EventPipeShadowStackSerializer::LogFunctionEvent(const FunctionEvent& event,
                                                         const DWORD& threadId,
                                                         std::map<FunctionID, FunctionInfo>& resolvedFunctions) {
    if (!resolvedFunctions.count(event.Id)) {
        resolvedFunctions[event.Id] = FunctionInfo::GetFunctionInfo(myProfilerInfo, event.Id);
    }

    auto eventPipeEvent = event.EventKind == FunctionEventKind::Started ? myMethodStartEvent : myMethodEndEvent;

    COR_PRF_EVENT_DATA eventData[3];

    eventData[0].ptr = reinterpret_cast<UINT64>(&event.Timestamp);
    eventData[0].size = sizeof(int64_t);

    eventData[1].ptr = reinterpret_cast<UINT64>(&event.Id);
    eventData[1].size = sizeof(FunctionID);

    eventData[2].ptr = reinterpret_cast<UINT64>(&threadId);
    eventData[2].size = sizeof(DWORD);

    auto dataCount = sizeof(eventData) / sizeof(COR_PRF_EVENT_DATA);

    return myProfilerInfo->EventPipeWriteEvent(eventPipeEvent, dataCount, eventData, NULL, NULL);
}

HRESULT EventPipeShadowStackSerializer::LogMethodInfo(const FunctionID& functionId, const FunctionInfo& functionInfo) {
    COR_PRF_EVENT_DATA eventData[2];

    eventData[0].ptr = reinterpret_cast<UINT64>(&functionId);
    eventData[0].size = sizeof(FunctionID);

    auto functionName = functionInfo.GetName();
    eventData[1].ptr = reinterpret_cast<UINT64>(&functionName);
    eventData[1].size = static_cast<UINT32>(functionName.length() + 1) * sizeof(WCHAR);

    auto dataCount = sizeof(eventData) / sizeof(COR_PRF_EVENT_DATA);

    return myProfilerInfo->EventPipeWriteEvent(myMethodInfoEvent, dataCount, eventData, NULL, NULL);
}