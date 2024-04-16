
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

    FunctionEvent(FunctionID id, FunctionEventKind eventKind, int64_t timestamp) :
            Id(id),
            EventKind(eventKind),
            Timestamp(timestamp) {
    }
};


void writeFunctionEvent(const FunctionEvent& event, std::ofstream& fout);

std::string createBinStackSavePath(const std::string& directory, const ThreadID& threadId);
#endif //PROCFILER_UTILS_H
