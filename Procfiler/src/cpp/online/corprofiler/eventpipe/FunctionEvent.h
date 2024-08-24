//
// Created by aeroo on 7/20/2024.
//

#ifndef FUNCTIONEVENT_H
#define FUNCTIONEVENT_H
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

    FunctionEvent(const FunctionID id, const FunctionEventKind eventKind, const int64_t timestamp) :
        Id(id),
        EventKind(eventKind),
        Timestamp(timestamp) {
    }
};


#endif //FUNCTIONEVENT_H
