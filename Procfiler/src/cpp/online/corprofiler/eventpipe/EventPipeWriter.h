#ifndef PROCFILER_EVENTPIPESHADOWSTACKSERIALIZER_H
#define PROCFILER_EVENTPIPESHADOWSTACKSERIALIZER_H

#include "cor.h"
#include "corprof.h"
#include <types.h>
#include <vector>
#include <map>
#include <string>
#include <stack>
#include <atomic>
#include <util.h>
#include <FunctionInfo.h>

#ifdef __linux__
#undef __pre
#endif

#include <regex>
#include "../util/logging/ProcfilerLogger.h"

#ifdef __linux__
#define __pre
#endif

struct FunctionEvent;
struct FunctionInfo;

class EventPipeWriter {
    const UINT32 ourMethodStartEventId = 8000;
    const UINT32 ourMethodEndEventId = 8001;
    const UINT32 ourMethodInfoEventId = 8002;
    const UINT32 outExceptionCatcherEnterEventId = 8003;

    const wstring ourMethodStartEventName = ToWString("ProcfilerMethod/Begin");
    const wstring ourMethodEndEventName = ToWString("ProcfilerMethod/End");
    const wstring ourMethodInfoEventName = ToWString("ProcfilerMethodInfo");
    const wstring ourEventPipeProviderName = ToWString("ProcfilerCppEventPipeProvider");
    const wstring ourExceptionCatcherEnterEventName = ToWString("ExceptionCatcher/Enter");

    const wstring ourTimestampMetadataKey = ToWString("Timestamp");
    const wstring ourFunctionIdMetadataKey = ToWString("FunctionId");
    const wstring ourFunctionNameMetadataKey = ToWString("FunctionName");

    std::regex* myMethodsFilterRegex{nullptr};

    ProcfilerLogger* myLogger;
    ICorProfilerInfo12 *myProfilerInfo;

    EVENTPIPE_PROVIDER myEventPipeProvider{};
    EVENTPIPE_EVENT myMethodStartEvent{};
    EVENTPIPE_EVENT myMethodEndEvent{};
    EVENTPIPE_EVENT myMethodInfoEvent{};
    EVENTPIPE_EVENT myExceptionCatcherEnterEvent{};

    HRESULT InitializeProvidersAndEvents();

    HRESULT DefineProcfilerEventPipeProvider();

    HRESULT DefineProcfilerMethodInfoEvent();

    HRESULT DefineProcfilerMethodStartEvent();

    HRESULT DefineProcfilerMethodEndEvent();

    HRESULT DefineProcfilerExceptionCatcherEnterEvent();

    HRESULT DefineMethodStartOrEndEventInternal(const wstring& eventName,
                                                EVENTPIPE_PROVIDER provider,
                                                EVENTPIPE_EVENT *ourEventId,
                                                ICorProfilerInfo12 *profilerInfo,
                                                UINT32 eventId) const;

    void InitMethodsFilterRegex();
    bool ShouldLogFunc(FunctionID functionId) const;

public:
    explicit EventPipeWriter(ICorProfilerInfo12 *profilerInfo);

    ~EventPipeWriter() = default;

    void Init();

    HRESULT LogFunctionEvent(const FunctionEvent &event) const;

    HRESULT LogMethodInfo(const FunctionID &functionId, const FunctionInfo &functionInfo) const;

    HRESULT LogExceptionCatcherEnterEvent(const FunctionID& functionId, const int64_t& timestamp) const;
};


#endif //PROCFILER_EVENTPIPESHADOWSTACKSERIALIZER_H
