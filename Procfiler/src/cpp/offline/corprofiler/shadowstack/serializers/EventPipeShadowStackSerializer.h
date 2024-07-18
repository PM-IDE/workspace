#ifndef PROCFILER_EVENTPIPESHADOWSTACKSERIALIZER_H
#define PROCFILER_EVENTPIPESHADOWSTACKSERIALIZER_H

#include "ShadowStackSerializer.h"

class EventPipeShadowStackSerializer : public ShadowStackSerializer {
private:
    const UINT32 ourMethodStartEventId = 8000;
    const UINT32 ourMethodEndEventId = 8001;
    const UINT32 ourMethodInfoEventId = 8002;

    const wstring ourMethodStartEventName = ToWString("ProcfilerMethodStart");
    const wstring ourMethodEndEventName = ToWString("ProcfilerMethodEnd");
    const wstring ourMethodInfoEventName = ToWString("ProcfilerMethodInfo");
    const wstring ourEventPipeProviderName = ToWString("ProcfilerCppEventPipeProvider");

    ICorProfilerInfo12* myProfilerInfo;
    ProcfilerLogger* myLogger;

    EVENTPIPE_PROVIDER myEventPipeProvider{};
    EVENTPIPE_EVENT myMethodStartEvent{};
    EVENTPIPE_EVENT myMethodEndEvent{};
    EVENTPIPE_EVENT myMethodInfoEvent{};

    HRESULT InitializeProvidersAndEvents();
    HRESULT DefineProcfilerEventPipeProvider();
    HRESULT DefineProcfilerMethodInfoEvent();

    HRESULT DefineProcfilerMethodStartEvent();
    HRESULT DefineProcfilerMethodEndEvent();

    HRESULT LogFunctionEvent(const FunctionEvent& event,
                             const DWORD& threadId,
                             std::map<FunctionID, FunctionInfo>& resolvedFunctions);

    HRESULT LogMethodInfo(const FunctionID& functionId, const FunctionInfo& functionInfo);


    static HRESULT DefineMethodStartOrEndEventInternal(const wstring& eventName,
                                                       EVENTPIPE_PROVIDER provider,
                                                       EVENTPIPE_EVENT* ourEventId,
                                                       ICorProfilerInfo12* profilerInfo,
                                                       UINT32 eventId);

public:
    explicit EventPipeShadowStackSerializer(ICorProfilerInfo12* profilerInfo, ProcfilerLogger* logger);
    ~EventPipeShadowStackSerializer() override = default;

    void Init() override;
    void Serialize(ShadowStack* shadowStack) override;
};


#endif //PROCFILER_EVENTPIPESHADOWSTACKSERIALIZER_H
