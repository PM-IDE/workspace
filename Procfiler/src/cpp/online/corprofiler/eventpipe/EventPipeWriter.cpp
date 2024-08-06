#include "EventPipeWriter.h"

#include <env_constants.h>
#include <FunctionInfo.h>
#include <util.h>

#include "FunctionEvent.h"

EventPipeWriter::EventPipeWriter(ICorProfilerInfo12 *profilerInfo) {
    myProfilerInfo = profilerInfo;
}

void EventPipeWriter::Init() {
    InitializeProvidersAndEvents();
    InitMethodsFilterRegex();
}

void EventPipeWriter::InitMethodsFilterRegex() {
    std::string value;
    if (TryGetEnvVar(filterMethodsRegex, value)) {
        try {
            myMethodsFilterRegex = new std::regex(value);
        }
        catch (const std::regex_error &e) {
            myMethodsFilterRegex = nullptr;
        }
    } else {
        myMethodsFilterRegex = nullptr;
    }
}

HRESULT EventPipeWriter::DefineProcfilerMethodStartEvent() {
    return DefineMethodStartOrEndEventInternal(ourMethodStartEventName,
                                               myEventPipeProvider,
                                               &myMethodStartEvent,
                                               myProfilerInfo,
                                               ourMethodStartEventId);
}

HRESULT EventPipeWriter::DefineProcfilerMethodEndEvent() {
    return DefineMethodStartOrEndEventInternal(ourMethodEndEventName,
                                               myEventPipeProvider,
                                               &myMethodEndEvent,
                                               myProfilerInfo,
                                               ourMethodEndEventId);
}

HRESULT EventPipeWriter::DefineProcfilerMethodInfoEvent() {
    COR_PRF_EVENTPIPE_PARAM_DESC eventParameters[] = {
        {COR_PRF_EVENTPIPE_UINT64, 0, ourFunctionIdMetadataKey.c_str()},
        {COR_PRF_EVENTPIPE_STRING, 0, ourFunctionNameMatadataKey.c_str()},
    };

    constexpr auto paramsCount = sizeof(eventParameters) / sizeof(COR_PRF_EVENTPIPE_PARAM_DESC);

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

HRESULT EventPipeWriter::DefineProcfilerEventPipeProvider() {
    return myProfilerInfo->EventPipeCreateProvider(ourEventPipeProviderName.c_str(), &myEventPipeProvider);
}

HRESULT EventPipeWriter::DefineMethodStartOrEndEventInternal(const wstring &eventName,
                                                             const EVENTPIPE_PROVIDER provider,
                                                             EVENTPIPE_EVENT *eventPipeEventId,
                                                             ICorProfilerInfo12 *profilerInfo,
                                                             const UINT32 eventId) {
    COR_PRF_EVENTPIPE_PARAM_DESC eventParameters[] = {
        {COR_PRF_EVENTPIPE_INT64, 0, ourTimestampMetadataKey.c_str()},
        {COR_PRF_EVENTPIPE_UINT64, 0, ourFunctionIdMetadataKey.c_str()},
    };

    constexpr auto paramsCount = sizeof(eventParameters) / sizeof(COR_PRF_EVENTPIPE_PARAM_DESC);

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
        eventPipeEventId
    );
}

HRESULT EventPipeWriter::InitializeProvidersAndEvents() {
    HRESULT hr;
    if ((hr = DefineProcfilerEventPipeProvider()) != S_OK) {
        return hr;
    }

    if ((hr = DefineProcfilerMethodStartEvent()) != S_OK) {
        return hr;
    }

    if ((hr = DefineProcfilerMethodEndEvent()) != S_OK) {
        return hr;
    }

    return S_OK;
}

static thread_local auto ourIgnoredFunctions = new std::map<FunctionID, bool>();

HRESULT EventPipeWriter::LogFunctionEvent(const FunctionEvent &event) {
    if (!ShouldLogFunc(event.Id)) {
        return S_OK;
    }

    const auto eventPipeEvent = event.EventKind == Started ? myMethodStartEvent : myMethodEndEvent;

    COR_PRF_EVENT_DATA eventData[2];

    eventData[0].ptr = reinterpret_cast<UINT64>(&event.Timestamp);
    eventData[0].size = sizeof(int64_t);

    eventData[1].ptr = reinterpret_cast<UINT64>(&event.Id);
    eventData[1].size = sizeof(FunctionID);

    constexpr auto dataCount = sizeof(eventData) / sizeof(COR_PRF_EVENT_DATA);

    return myProfilerInfo->EventPipeWriteEvent(eventPipeEvent, dataCount, eventData, nullptr, nullptr);
}

bool EventPipeWriter::ShouldLogFunc(FunctionID functionId) {
    if (myMethodsFilterRegex == nullptr) {
        return true;
    }

    if (ourIgnoredFunctions->find(functionId) != ourIgnoredFunctions->end()) {
        return ourIgnoredFunctions->at(functionId);
    }

    const auto functionName = FunctionInfo::GetFunctionInfo(myProfilerInfo, functionId).GetFullName();

    std::smatch m;
    auto shouldLog = std::regex_search(functionName, m, *myMethodsFilterRegex);

    ourIgnoredFunctions->insert({functionId, shouldLog});

    return shouldLog;
}

HRESULT EventPipeWriter::LogMethodInfo(const FunctionID &functionId, const FunctionInfo &functionInfo) const {
    COR_PRF_EVENT_DATA eventData[2];

    eventData[0].ptr = reinterpret_cast<UINT64>(&functionId);
    eventData[0].size = sizeof(FunctionID);

    auto functionName = functionInfo.GetName();
    eventData[1].ptr = reinterpret_cast<UINT64>(&functionName);
    eventData[1].size = static_cast<UINT32>(functionName.length() + 1) * sizeof(WCHAR);

    constexpr auto dataCount = sizeof(eventData) / sizeof(COR_PRF_EVENT_DATA);

    return myProfilerInfo->EventPipeWriteEvent(myMethodInfoEvent, dataCount, eventData, nullptr, nullptr);
}
