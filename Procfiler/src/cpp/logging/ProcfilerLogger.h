#include "iostream"

#ifndef PROCFILER_PROCFILER_LOGGER_H
#define PROCFILER_PROCFILER_LOGGER_H

class ProcfilerLogger {
private:
    bool myIsEnabled;

public:
    ProcfilerLogger();

    void LogInformation(const std::string& message);
    void LogError(const std::string& message);
};


#endif //PROCFILER_PROCFILER_LOGGER_H
