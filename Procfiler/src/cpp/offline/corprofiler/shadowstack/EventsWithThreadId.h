#ifndef PROCFILER_EVENTSWITHTHREADID_H
#define PROCFILER_EVENTSWITHTHREADID_H

#include "cor.h"
#include "corprof.h"
#include <types.h>
#include "../../logging/ProcfilerLogger.h"
#include <vector>
#include <map>
#include <string>
#include <stack>
#include <atomic>
#include <regex>
#include "../info/FunctionInfo.h"
#include "fstream"
#include "utils.h"

struct EventsWithThreadId {
    std::stack<FunctionID>* CurrentStack = new std::stack<FunctionID>();
    std::map<FunctionID, bool>* ShouldLogFuncs = new std::map<FunctionID, bool>();
    int64_t lastEventStamp{0};

    virtual ~EventsWithThreadId() = default;

    void AddFunctionEvent(const FunctionEvent& event);

    virtual void AddFunctionEventBypassStack(const FunctionEvent& event) = 0;

    bool ShouldLog(FunctionID& id, bool* shouldLog) const;

    void PutFunctionShouldLogDecision(FunctionID& id, bool shouldLog) const;
};

struct EventsWithThreadIdOnline : public EventsWithThreadId {
    std::ofstream* myFout{nullptr};
    std::ofstream::pos_type myFramesCountPos{};
    long long writtenFrames{0};

    explicit EventsWithThreadIdOnline(std::string& directory, ThreadID threadId);

    ~EventsWithThreadIdOnline() override;

    void AddFunctionEventBypassStack(const FunctionEvent& event) override;
};


struct EventsWithThreadIdOffline : public EventsWithThreadId {
    std::vector<FunctionEvent>* Events{nullptr};

    explicit EventsWithThreadIdOffline();

    ~EventsWithThreadIdOffline() override;

    void AddFunctionEventBypassStack(const FunctionEvent& event) override;
};


#endif //PROCFILER_EVENTSWITHTHREADID_H
