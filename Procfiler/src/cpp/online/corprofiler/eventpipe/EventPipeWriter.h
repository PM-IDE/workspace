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

    const wstring ourMethodStartEventName = ToWString("ProcfilerMethodStart");
    const wstring ourMethodEndEventName = ToWString("ProcfilerMethodEnd");
    const wstring ourMethodInfoEventName = ToWString("ProcfilerMethodInfo");
    const wstring ourEventPipeProviderName = ToWString("ProcfilerCppEventPipeProvider");

    ICorProfilerInfo12 *myProfilerInfo;

    EVENTPIPE_PROVIDER myEventPipeProvider{};
    EVENTPIPE_EVENT myMethodStartEvent{};
    EVENTPIPE_EVENT myMethodEndEvent{};
    EVENTPIPE_EVENT myMethodInfoEvent{};

    HRESULT InitializeProvidersAndEvents();

    HRESULT DefineProcfilerEventPipeProvider();

    HRESULT DefineProcfilerMethodInfoEvent();

    HRESULT DefineProcfilerMethodStartEvent();

    HRESULT DefineProcfilerMethodEndEvent();

    static HRESULT DefineMethodStartOrEndEventInternal(const wstring &eventName,
                                                       EVENTPIPE_PROVIDER provider,
                                                       EVENTPIPE_EVENT *ourEventId,
                                                       ICorProfilerInfo12 *profilerInfo,
                                                       UINT32 eventId);

public:
    explicit EventPipeWriter(ICorProfilerInfo12 *profilerInfo);

    ~EventPipeWriter() = default;

    void Init();

    HRESULT LogFunctionEvent(const FunctionEvent &event) const;

    HRESULT LogMethodInfo(const FunctionID &functionId, const FunctionInfo &functionInfo) const;
};


#endif //PROCFILER_EVENTPIPESHADOWSTACKSERIALIZER_H
