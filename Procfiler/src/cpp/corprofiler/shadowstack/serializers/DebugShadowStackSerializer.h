#ifndef PROCFILER_DEBUGSHADOWSTACKSERIALIZER_H
#define PROCFILER_DEBUGSHADOWSTACKSERIALIZER_H

#include "ShadowStackSerializer.h"

class DebugShadowStackSerializer : public ShadowStackSerializer {
private:
    std::string mySavePath;
    ICorProfilerInfo12* myProfilerInfo;
    ProcfilerLogger* myLogger;

public:
    explicit DebugShadowStackSerializer(ICorProfilerInfo12* profilerInfo, ProcfilerLogger* logger);
    ~DebugShadowStackSerializer() override = default;

    void Init() override;
    void Serialize(ShadowStack* shadowStack) override;
};

#endif //PROCFILER_DEBUGSHADOWSTACKSERIALIZER_H
