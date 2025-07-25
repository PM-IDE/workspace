#ifndef PROCFILER_BINARYSHADOWSTACKSERIALIZER_H
#define PROCFILER_BINARYSHADOWSTACKSERIALIZER_H

#include <set>
#include "ShadowStackSerializer.h"
#include "../EventsWithThreadId.h"

class BinaryShadowStackSerializer : public ShadowStackSerializer {
private:
    std::string mySavePath;
    ICorProfilerInfo15* myProfilerInfo;
    ProcfilerLogger* myLogger;
    bool myUseSeparateBinstacksFiles{false};

    void SerializeInSingleFile(ShadowStack* shadowStack);
    void SerializeInDifferentFiles(ShadowStack* shadowStack);

    void WriteThreadStack(ThreadID threadId,
                          std::vector<FunctionEvent>* events,
                          std::ofstream& fout,
                          std::set<FunctionID>& filteredOutFunctions,
                          std::regex* methodsFilterRegex);

    std::regex* TryCreateMethodsFilterRegex();
public:
    explicit BinaryShadowStackSerializer(ICorProfilerInfo15* profilerInfo, ProcfilerLogger* logger);
    ~BinaryShadowStackSerializer() override = default;

    void Init() override;
    void Serialize(ShadowStack* shadowStack) override;
};


#endif //PROCFILER_BINARYSHADOWSTACKSERIALIZER_H
