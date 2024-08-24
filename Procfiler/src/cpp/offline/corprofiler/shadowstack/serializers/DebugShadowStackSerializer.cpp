#include <fstream>
#include "DebugShadowStackSerializer.h"
#include <env_constants.h>

DebugShadowStackSerializer::DebugShadowStackSerializer(ICorProfilerInfo12* profilerInfo, ProcfilerLogger* logger) {
    myProfilerInfo = profilerInfo;
    myLogger = logger;
}

void DebugShadowStackSerializer::Init() {
    if (!TryGetEnvVar(shadowStackDebugSavePath, this->mySavePath)) {
        myLogger->LogError("Debug shadow stack save path was not defined");
        return;
    }
}

void DebugShadowStackSerializer::Serialize(ShadowStack* shadowStack) {
    if (mySavePath.length() == 0) {
        return;
    }

    std::ofstream fout(mySavePath);
    std::map<FunctionID, FunctionInfo> resolvedFunctions;
    const std::string startPrefix = "[START]: ";
    const std::string endPrefix = "[ END ]: ";

    for (const auto& pair: *(shadowStack->GetAllStacks())) {
        auto offlineEvents = dynamic_cast<EventsWithThreadIdOffline*>(pair.second);
        if (offlineEvents == nullptr) continue;

        auto threadFrame = "Thread(" + std::to_string(pair.first) + ")\n";
        fout << startPrefix << threadFrame;
        auto indent = 1;

        for (auto event: *(offlineEvents->Events)) {
            if (!resolvedFunctions.count(event.Id)) {
                resolvedFunctions[event.Id] = FunctionInfo::GetFunctionInfo(myProfilerInfo, event.Id);
            }

            auto& functionInfo = resolvedFunctions[event.Id];
            const std::string indentString = "  ";
            std::string funcName;

            if (event.EventKind == FunctionEventKind::Finished) {
                --indent;
            }

            for (int i = 0; i < indent; ++i) {
                funcName += indentString;
            }

            auto prefix = event.EventKind == FunctionEventKind::Started ? startPrefix : endPrefix;
            funcName += prefix + functionInfo.GetFullName() + "\n";
            fout << funcName;

            if (event.EventKind == FunctionEventKind::Started) {
                ++indent;
            }
        }

        fout << endPrefix << threadFrame;
        fout << "\n\n\n";
    }

    fout.close();
}
