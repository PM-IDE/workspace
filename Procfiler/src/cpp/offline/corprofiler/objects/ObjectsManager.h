#ifndef PROCFILER_OBJECTSMANAGER_H
#define PROCFILER_OBJECTSMANAGER_H
#include "cor.h"
#include "corprof.h"

class ObjectsManager {
    ICorProfilerInfo15* myProfilerInfo;

public:
    explicit ObjectsManager(ICorProfilerInfo15* info);

    ~ObjectsManager();

    bool TryGetThisObjectId(FunctionID funcId, const COR_PRF_FUNCTION_ARGUMENT_INFO* args, ObjectID* objectId) const;
};


#endif //PROCFILER_OBJECTSMANAGER_H
