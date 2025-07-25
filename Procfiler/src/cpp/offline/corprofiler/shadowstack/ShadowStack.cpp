#include "ShadowStack.h"
#include <env_constants.h>
#include <mutex>
#include <stack>
#include "clr/profiler.hpp"

static thread_local bool ourIsInitialized = false;
static thread_local EventsWithThreadId* ourEvents;
static std::map<ThreadID, EventsWithThreadId*> ourEventsPerThreads;
static std::mutex ourEventsPerThreadMutex;

ShadowStack::ShadowStack(ICorProfilerInfo15* profilerInfo, ProcfilerLogger* logger, bool onlineSerialization) {
    myProfilerInfo = profilerInfo;
    myOnlineSerialization = onlineSerialization;

    if (IsEnvVarTrue(filterMethodsDuringRuntime)) {
        std::string value;
        if (TryGetEnvVar(filterMethodsRegex, value)) {
            try {
                myFilterRegex = new std::regex(value);
            }
            catch (const std::regex_error &e) {
                myFilterRegex = nullptr;
            }
        } else {
            myFilterRegex = nullptr;
        }
    }

    myLogger = logger;
}

ShadowStack::~ShadowStack() {
    for (auto& pair: *GetAllStacks()) {
        delete pair.second;
    }
}

void ShadowStack::AddFunctionEnter(FunctionID id, DWORD threadId, int64_t timestamp) {
    if (!CanProcessFunctionEvents()) return;

    FunctionEventProcessingCookie cookie(&this->myCurrentAddition);

    if (!CanProcessFunctionEvents()) return;

    if (ShouldAddFunc(id, threadId)) {
        const auto event = FunctionEvent(id, FunctionEventKind::Started, timestamp);
        GetOrCreatePerThreadEvents(myLogger, threadId, myOnlineSerialization)->AddFunctionEvent(event);
    }
}

bool ShadowStack::ShouldAddFunc(FunctionID& id, DWORD threadId) {
    if (myFilterRegex == nullptr) return true;

    const auto events = GetOrCreatePerThreadEvents(myLogger, threadId, myOnlineSerialization);

    bool shouldLog;
    if (events->ShouldLog(id, &shouldLog)) {
        return shouldLog;
    }

    const auto functionName = ToString(GetFullFunctionName(id, myProfilerInfo));

    std::smatch m;
    shouldLog = std::regex_search(functionName, m, *myFilterRegex);
    events->PutFunctionShouldLogDecision(id, shouldLog);
    return shouldLog;
}

void ShadowStack::AddFunctionFinished(FunctionID id, DWORD threadId, int64_t timestamp) {
    if (!CanProcessFunctionEvents()) return;

    FunctionEventProcessingCookie cookie(&this->myCurrentAddition);

    if (!CanProcessFunctionEvents()) return;

    if (ShouldAddFunc(id, threadId)) {
        const auto event = FunctionEvent(id, FunctionEventKind::Finished, timestamp);
        GetOrCreatePerThreadEvents(myLogger, threadId, myOnlineSerialization)->AddFunctionEvent(event);
    }
}

void ShadowStack::HandleExceptionCatchEnter(FunctionID catcherFunctionId, DWORD threadId, int64_t timestamp) {
    if (!CanProcessFunctionEvents()) return;

    FunctionEventProcessingCookie cookie(&this->myCurrentAddition);

    if (!CanProcessFunctionEvents()) return;

    auto events = GetOrCreatePerThreadEvents(myLogger, threadId, myOnlineSerialization);
    auto stack = events->CurrentStack;
    while (!stack->empty()) {
        auto top = stack->top();
        if (top == catcherFunctionId) {
            break;
        }

        //stack is popped in this function (AddFunctionEvent + FunctionEventKind::Finished)
        events->AddFunctionEvent(FunctionEvent(top, FunctionEventKind::Finished, timestamp));
    }
}

EventsWithThreadId* ShadowStack::GetOrCreatePerThreadEvents(ProcfilerLogger* logger, DWORD threadId, bool onlineSerialization) {
    if (!ourIsInitialized) {
        std::unique_lock<std::mutex> lock{ourEventsPerThreadMutex};
        if (!ourIsInitialized) {
            if (onlineSerialization) {
                std::string directory;
                if (!TryGetEnvVar(binaryStackSavePath, directory)) {
                    logger->LogError("The save path directory was not specified");
                }

                logger->LogInformation("Using online serialization");
                ourEvents = new EventsWithThreadIdOnline(directory, threadId);
            } else {
                logger->LogInformation("Using offline serialization");
                ourEvents = new EventsWithThreadIdOffline();
            }

            ourEventsPerThreads[threadId] = ourEvents;
            ourIsInitialized = true;
        }
    }

    return ourEvents;
}

std::map<ThreadID, EventsWithThreadId*>* ShadowStack::GetAllStacks() const {
    return &ourEventsPerThreads;
}

void ShadowStack::AdjustShadowStacks() {
    if (myCanProcessFunctionEvents.load(std::memory_order_seq_cst)) {
        myLogger->LogError("Can not adjust stack while additions are still allowed");
        return;
    }

    for (const auto& pair : *GetAllStacks()) {
        auto events = pair.second;
        auto stack = events->CurrentStack;

        while (!stack->empty()) {
            const auto& top = stack->top();
            events->AddFunctionEventBypassStack(FunctionEvent(top, FunctionEventKind::Finished, events->lastEventStamp));
            stack->pop();
        }
    }
}

void ShadowStack::SuppressFurtherMethodsEvents() {
    myCanProcessFunctionEvents.store(false, std::memory_order_seq_cst);
}

void ShadowStack::WaitForPendingMethodsEvents() {
    while (myCurrentAddition.load(std::memory_order_seq_cst) != 0) {
    }
}

bool ShadowStack::CanProcessFunctionEvents() {
    return myCanProcessFunctionEvents.load(std::memory_order_seq_cst);
}