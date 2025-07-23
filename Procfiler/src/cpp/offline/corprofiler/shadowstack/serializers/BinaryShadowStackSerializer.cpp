#include "BinaryShadowStackSerializer.h"
#include <env_constants.h>
#include <fstream>
#include <regex>
#include <set>

BinaryShadowStackSerializer::BinaryShadowStackSerializer(ICorProfilerInfo15* profilerInfo,
                                                         ProcfilerLogger* logger) {
    myProfilerInfo = profilerInfo;
    myLogger = logger;
}

void BinaryShadowStackSerializer::Init() {
    if (!TryGetEnvVar(binaryStackSavePath, this->mySavePath)) {
        myLogger->LogError("Binary shadow stack save path was not defined");
    }

    std::string value;
    TryGetEnvVar(useSeparateBinStacksFiles, value);

    myUseSeparateBinstacksFiles = value == trueEnvVarValue;
}

void BinaryShadowStackSerializer::Serialize(ShadowStack* shadowStack) {
    if (mySavePath.length() == 0) {
        myLogger->LogError("Will not serialize shadow stack to binary format as save path was not provided");
        return;
    }

    if (myUseSeparateBinstacksFiles) {
        SerializeInDifferentFiles(shadowStack);
    } else {
        SerializeInSingleFile(shadowStack);
    }

    myLogger->LogInformation("Finished serializing shadow stack to binary file");
}

void BinaryShadowStackSerializer::WriteThreadStack(ThreadID threadId,
                                                   std::vector<FunctionEvent>* events,
                                                   std::ofstream& fout,
                                                   std::set<FunctionID>& filteredOutFunctions,
                                                   std::regex* methodsFilterRegex) {
    fout.write((char*) &threadId, sizeof(long long));

    auto framesCount = (long long) events->size();
    auto countPos = fout.tellp();
    fout.write((char*) &framesCount, sizeof(long long));

    long long writtenFrames = 0;
    for (const auto& event: *events) {
        if (filteredOutFunctions.find(event.Id) != filteredOutFunctions.end()) {
            continue;
        }

        auto shouldSkip = false;
        if (methodsFilterRegex != nullptr) {
            auto functionName = FunctionInfo::GetFunctionInfo(myProfilerInfo, event.Id).GetFullName();
            std::smatch m;
            if (!std::regex_search(functionName, m, *methodsFilterRegex)) {
                shouldSkip = true;
                filteredOutFunctions.insert(event.Id);
            }
        }

        if (shouldSkip) {
            continue;
        }

        writeFunctionEvent(event, fout);
        ++writtenFrames;
    }

    auto streamPos = fout.tellp();
    fout.seekp(countPos);
    fout.write((char*) &writtenFrames, sizeof(long long));
    fout.seekp(streamPos);
}

std::regex* BinaryShadowStackSerializer::TryCreateMethodsFilterRegex() {
    std::string methodFilterRegex;
    std::regex* regex = nullptr;
    if (TryGetEnvVar(filterMethodsRegex, methodFilterRegex)) {
        try {
            myLogger->LogInformation("Creating regex from " + methodFilterRegex);
            regex = new std::regex(methodFilterRegex);
        }
        catch (const std::regex_error& e) {
            myLogger->LogError("Failed to create regex from " + methodFilterRegex);
            regex = nullptr;
        }
    }

    return regex;
}

void BinaryShadowStackSerializer::SerializeInSingleFile(ShadowStack* shadowStack) {
    myLogger->LogInformation("Started serializing shadow stack to binary file");
    std::ofstream fout(mySavePath, std::ios::binary);
    std::set<FunctionID> filteredOutFunctions;
    std::regex* regex = TryCreateMethodsFilterRegex();

    for (auto& pair: *(shadowStack->GetAllStacks())) {
        auto offlineEvents = dynamic_cast<EventsWithThreadIdOffline*>(pair.second);
        if (offlineEvents != nullptr) {
            WriteThreadStack(pair.first, offlineEvents->Events, fout, filteredOutFunctions, regex);
        }
    }

    fout.close();
    delete regex;
}

void BinaryShadowStackSerializer::SerializeInDifferentFiles(ShadowStack* shadowStack) {
    myLogger->LogInformation("Started serializing shadow stack to several binary files");
    std::set<FunctionID> filteredOutFunctions;
    std::regex* regex = TryCreateMethodsFilterRegex();

    for (auto& pair: *(shadowStack->GetAllStacks())) {
        auto offlineEvents = dynamic_cast<EventsWithThreadIdOffline*>(pair.second);

        if (offlineEvents != nullptr) {
            std::string filePath = createBinStackSavePath(mySavePath, pair.first);
            std::ofstream fout(filePath, std::ios::binary);
            WriteThreadStack(pair.first, offlineEvents->Events, fout, filteredOutFunctions, regex);

            fout.close();
        }
    }

    delete regex;
}
