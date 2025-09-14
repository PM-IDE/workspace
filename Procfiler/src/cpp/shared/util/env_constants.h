#ifndef PROCFILER_ENV_VARS_H
#define PROCFILER_ENV_VARS_H

#include <string>

const std::string trueEnvVarValue = "1";

const std::string shadowStackDebugSavePathEnv = "PROCFILER_DEBUG_SAVE_CALL_STACKS_PATH";
const std::string enableConsoleLoggingEnv = "PROCFILER_ENABLE_CONSOLE_LOGGING";
const std::string binaryStackSavePathEnv = "PROCFILER_BINARY_SAVE_STACKS_PATH";
const std::string eventPipeSaveShadowStackEnv = "PROCFILER_EVENT_PIPE_SAVE_STACKS";

const std::string filterMethodsRegexEnv = "PROCFILER_FILTER_METHODS_REGEX";
const std::string filterMethodsDuringRuntimeEnv = "PROCFILER_FILTER_METHODS_DURING_RUNTIME";
const std::string useSeparateBinStacksFilesEnv = "PROCFILER_USE_SEPARATE_BINSTACKS_FILES";
const std::string onlineSerializationEnv = "PROCFILER_ONLINE_SERIALIZATION";

const std::string produceObjectBinStacksEnv = "PROCFILER_PRODUCE_OBJECT_BIN_STACKS";
const std::string objectsTrackingFilterRegex = "PROCFILER_OBJECTS_TRACKING_FILTER_REGEX";


bool IsEnvVarDefined(const std::string& envVarName);
bool TryGetEnvVar(const std::string& envVarName, std::string& value);
bool IsEnvVarTrue(const std::string& envVarName);

#endif