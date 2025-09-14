#ifndef PROCFILER_OBJECTSMANAGER_H
#define PROCFILER_OBJECTSMANAGER_H

#include <map>
#include <mutex>

#include "cor.h"
#include "corprof.h"

class ObjectsManager {
    ICorProfilerInfo15* myProfilerInfo;
    std::map<ObjectID, UINT64> myObjectsIds;
    std::mutex myMutex;

public:
    explicit ObjectsManager(ICorProfilerInfo15* info);

    ~ObjectsManager();

    bool TryGetThisObjectId(FunctionID funcId, const COR_PRF_FUNCTION_ARGUMENT_INFO* args, UINT64* objectPtr);

    void HandleObjectsMove(ULONG cMovedObjectIDRanges,
                           ObjectID* oldObjectIDRangeStart,
                           ObjectID* newObjectIDRangeStart,
                           ULONG* cObjectIDRangeLength);

    void HandleObjectAllocation(const ObjectID& id);
};


#endif //PROCFILER_OBJECTSMANAGER_H
