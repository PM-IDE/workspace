#include "env_constants.h"

bool IsEnvVarDefined(const std::string& envVarName) {
    return std::getenv(envVarName.c_str()) != nullptr;
}

bool TryGetEnvVar(const std::string& envVarName, std::string& value) {
    const auto envVar = std::getenv(envVarName.c_str());
    const bool isEnvVarDefined = envVar != nullptr;
    value = isEnvVarDefined ? std::string(envVar) : "";
    return isEnvVarDefined;
}

bool IsEnvVarTrue(const std::string& envVarName) {
    std::string value;
    if (!TryGetEnvVar(envVarName, value)) {
        return false;
    }

    return value == trueEnvVarValue;
}