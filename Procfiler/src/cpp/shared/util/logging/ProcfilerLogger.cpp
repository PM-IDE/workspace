#include <cstdio>
#include "ProcfilerLogger.h"
#include "../util/env_constants.h"

char* PatchMessage(const std::string& message) {
    auto patchedMessage = new char[message.length() + 1];
    auto i = 0;
    for (;i < message.length(); ++i) {
        patchedMessage[i] = message[i];
    }

    patchedMessage[i] = '\n';
    return patchedMessage;
}

void ProcfilerLogger::LogInformation(const std::string& message) {
    if (!myIsEnabled) return;

    auto patchedMessage = PatchMessage(message);

    std::cout << patchedMessage;
    delete[] patchedMessage;
}

ProcfilerLogger::ProcfilerLogger() {
    auto enableLoggingEnv = std::getenv(enableConsoleLoggingEnv.c_str());
    myIsEnabled = enableLoggingEnv != nullptr && std::string(enableLoggingEnv) == "1";
}

void ProcfilerLogger::LogError(const std::string &message) {
    if (!myIsEnabled) return;

    auto patchedMessage = PatchMessage(message);

    std::cerr << patchedMessage;
    delete[] patchedMessage;
}
