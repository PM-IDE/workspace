#ifndef PROCFILER_UTILS_H
#define PROCFILER_UTILS_H

#include "fstream"
#include "cor.h"
#include "corprof.h"

enum FunctionEventKind {
    Started,
    Finished
};

struct FunctionEvent {
    FunctionID Id;
    FunctionEventKind EventKind;
    int64_t Timestamp;

    FunctionEvent(const FunctionID id, const FunctionEventKind eventKind, const int64_t timestamp) : Id(id),
        EventKind(eventKind),
        Timestamp(timestamp) {
    }
};

struct ObjectFunctionEvent : FunctionEvent {
    int64_t ObjectId;
    int64_t TypeId;

    ObjectFunctionEvent(
        const FunctionID id,
        const FunctionEventKind eventKind,
        const int64_t timestamp,
        const int64_t objectId,
        const int64_t typeId
    ) : FunctionEvent(id, eventKind, timestamp), ObjectId(objectId), TypeId(typeId) {
    }
};

void writeFunctionEvent(const FunctionEvent& event, std::ofstream& fout);

std::string createBinStackSavePath(const std::string& directory, const ThreadID& threadId);
#endif //PROCFILER_UTILS_H
