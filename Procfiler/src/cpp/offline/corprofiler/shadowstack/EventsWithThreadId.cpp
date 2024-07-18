#include "EventsWithThreadId.h"

void EventsWithThreadId::AddFunctionEvent(const FunctionEvent& event) {
    lastEventStamp = event.Timestamp;

    if (event.EventKind == FunctionEventKind::Started) {
        CurrentStack->push(event.Id);
    } else {
        CurrentStack->pop();
    }

    AddFunctionEventBypassStack(event);
}

bool EventsWithThreadId::ShouldLog(FunctionID& id, bool* shouldLog) const {
    if (ShouldLogFuncs->find(id) == ShouldLogFuncs->end()) {
        return false;
    } else {
        *shouldLog = ShouldLogFuncs->at(id);
        return true;
    }
}

void EventsWithThreadId::PutFunctionShouldLogDecision(FunctionID& id, bool shouldLog) const {
    ShouldLogFuncs->insert({id, shouldLog});
}

EventsWithThreadIdOnline::EventsWithThreadIdOnline(std::string& directory, ThreadID threadId) {
    std::string savePath = directory + "binstack_" + std::to_string(threadId) + ".bin";
    myFout = new std::ofstream(savePath, std::ios::binary);

    myFout->write((char*) &threadId, sizeof(long long));

    myFramesCountPos = myFout->tellp();
    auto initialCount = 0;
    myFout->write((char*) &initialCount, sizeof(long long));
}

EventsWithThreadIdOnline::~EventsWithThreadIdOnline() {
    auto streamPos = myFout->tellp();
    myFout->seekp(myFramesCountPos);
    myFout->write((char*) &writtenFrames, sizeof(long long));
    myFout->seekp(streamPos);

    myFout->close();
    delete myFout;
}

void EventsWithThreadIdOnline::AddFunctionEventBypassStack(const FunctionEvent& event) {
    ++writtenFrames;
    writeFunctionEvent(event, *myFout);
}

EventsWithThreadIdOffline::EventsWithThreadIdOffline() {
    ShouldLogFuncs = new std::map<FunctionID, bool>;
    Events = new std::vector<FunctionEvent>();
}

EventsWithThreadIdOffline::~EventsWithThreadIdOffline() {
    delete Events;
}

void EventsWithThreadIdOffline::AddFunctionEventBypassStack(const FunctionEvent& event) {
    Events->push_back(event);
}