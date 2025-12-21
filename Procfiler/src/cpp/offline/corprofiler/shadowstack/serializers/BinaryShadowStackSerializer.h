#ifndef PROCFILER_BINARYSHADOWSTACKSERIALIZER_H
#define PROCFILER_BINARYSHADOWSTACKSERIALIZER_H

#include <set>
#include "ShadowStackSerializer.h"
#include "../EventsWithThreadId.h"

class BinaryShadowStackSerializer : public ShadowStackSerializer {
    std::string mySavePath;
    ICorProfilerInfo15* myProfilerInfo;
    ProcfilerLogger* myLogger;
    bool myUseSeparateBinstacksFiles{false};

    void SerializeInSingleFile(const ShadowStack* shadowStack) const;
    void SerializeInDifferentFiles(const ShadowStack* shadowStack) const;

    void WriteThreadStack(ThreadID threadId,
                          const std::vector<FunctionEvent>* events,
                          std::ofstream& fout,
                          std::set<FunctionID>& filteredOutFunctions,
                          const std::regex* methodsFilterRegex) const;

    std::regex* TryCreateMethodsFilterRegex() const;
public:
    explicit BinaryShadowStackSerializer(ICorProfilerInfo15* profilerInfo, ProcfilerLogger* logger);
    ~BinaryShadowStackSerializer() override = default;

    void Init() override;
    void Serialize(ShadowStack* shadowStack) override;
};


#endif //PROCFILER_BINARYSHADOWSTACKSERIALIZER_H
