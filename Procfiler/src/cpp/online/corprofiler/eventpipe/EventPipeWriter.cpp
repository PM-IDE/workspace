#include "EventPipeWriter.h"

#include <FunctionInfo.h>
#include <map>
#include <util.h>

#include "FunctionEvent.h"

EventPipeWriter::EventPipeWriter(ICorProfilerInfo12 *profilerInfo) {
    myProfilerInfo = profilerInfo;
}

void EventPipeWriter::Init() {
    InitializeProvidersAndEvents();
}

HRESULT EventPipeWriter::DefineProcfilerMethodStartEvent() {
    return DefineMethodStartOrEndEventInternal(ToWString("ProcfilerMethodStart"),
                                               myEventPipeProvider,
                                               &myMethodStartEvent,
                                               myProfilerInfo,
                                               ourMethodStartEventId);
}

HRESULT EventPipeWriter::DefineProcfilerMethodEndEvent() {
    return DefineMethodStartOrEndEventInternal(ToWString("ProcfilerMethodEnd"),
                                               myEventPipeProvider,
                                               &myMethodEndEvent,
                                               myProfilerInfo,
                                               ourMethodEndEventId);
}

HRESULT EventPipeWriter::DefineProcfilerMethodInfoEvent() {
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

HRESULT EventPipeWriter::DefineProcfilerEventPipeProvider() {
    return myProfilerInfo->EventPipeCreateProvider(ourEventPipeProviderName.c_str(), &myEventPipeProvider);
}

HRESULT EventPipeWriter::DefineMethodStartOrEndEventInternal(const wstring &eventName,
                                                             EVENTPIPE_PROVIDER provider,
                                                             EVENTPIPE_EVENT *outEventId,
                                                             ICorProfilerInfo12 *profilerInfo,
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

    if ((hr = DefineProcfilerMethodInfoEvent()) != S_OK) {
        return hr;
    }

    return S_OK;
}

HRESULT EventPipeWriter::LogFunctionEvent(const FunctionEvent &event, const DWORD &threadId) {
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

HRESULT EventPipeWriter::LogMethodInfo(const FunctionID &functionId, const FunctionInfo &functionInfo) {
    COR_PRF_EVENT_DATA eventData[2];

    eventData[0].ptr = reinterpret_cast<UINT64>(&functionId);
    eventData[0].size = sizeof(FunctionID);

    auto functionName = functionInfo.GetName();
    eventData[1].ptr = reinterpret_cast<UINT64>(&functionName);
    eventData[1].size = static_cast<UINT32>(functionName.length() + 1) * sizeof(WCHAR);

    auto dataCount = sizeof(eventData) / sizeof(COR_PRF_EVENT_DATA);

    return myProfilerInfo->EventPipeWriteEvent(myMethodInfoEvent, dataCount, eventData, NULL, NULL);
}
