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
#include <regex>
#include <util.h>
#include <FunctionInfo.h>

struct FunctionEvent;
struct FunctionInfo;

class EventPipeWriter {
private:
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
                                                UINT32 eventId);

    void InitMethodsFilterRegex();
    bool ShouldLogFunc(FunctionID functionId);

public:
    explicit EventPipeWriter(ICorProfilerInfo12 *profilerInfo);

    ~EventPipeWriter() = default;

    void Init();

    HRESULT LogFunctionEvent(const FunctionEvent &event);

    HRESULT LogMethodInfo(const FunctionID &functionId, const FunctionInfo &functionInfo) const;
};


#endif //PROCFILER_EVENTPIPESHADOWSTACKSERIALIZER_H
