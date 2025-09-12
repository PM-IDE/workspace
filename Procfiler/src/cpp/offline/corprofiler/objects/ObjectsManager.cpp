#include "ObjectsManager.h"

#include "FunctionInfo.h"
#include "sigparser/sigparserimpl.hpp"
#include <cor.h>
#include <corhdr.h>

ObjectsManager::ObjectsManager(ICorProfilerInfo15* info) {
    myProfilerInfo = info;
}

ObjectsManager::~ObjectsManager() {
    myProfilerInfo = nullptr;
}

bool ObjectsManager::TryGetThisObjectId(const FunctionID funcId,
                                        const COR_PRF_FUNCTION_ARGUMENT_INFO* args,
                                        ObjectID* objectId) const {
    if (args->ranges->length == 0) {
        return false;
    }

    mdToken functionToken;
    ClassID classId;
    ModuleID moduleId;

    auto hr = myProfilerInfo->GetFunctionInfo(funcId, &classId, &moduleId, &functionToken);
    if (FAILED(hr)) {
        return false;
    }

    IUnknown* unknown;
    hr = myProfilerInfo->GetModuleMetaData(moduleId, ofRead | ofWrite, IID_IMetaDataImport, &unknown);
    if (FAILED(hr)) {
        return false;
    }

    IMetaDataImport2* mtd = nullptr;
    auto ptr = reinterpret_cast<void**>(&mtd);
    hr = unknown->QueryInterface(IID_IMetaDataImport, ptr);
    if (FAILED(hr)) {
        return false;
    }

    PCCOR_SIGNATURE signature;
    ULONG signatureLength;

    if (FAILED(mtd->GetMethodProps(functionToken, 0, 0, 0, 0, 0, &signature, &signatureLength, 0, 0))) {
        return false;
    }

    SigFormatParserImpl sigParser;
    if (!sigParser.Parse(const_cast<sig_byte*>(signature), signatureLength) || !sigParser.HasThis()) {
        return false;
    }

    const auto thisId = reinterpret_cast<UINT_PTR>(*reinterpret_cast<void**>(args->ranges[0].startAddress));

    COR_PRF_GC_GENERATION_RANGE generationRange;
    if (FAILED(myProfilerInfo->GetObjectGeneration(thisId, &generationRange))) {
        return false;
    }

    *objectId = thisId;

    return true;
}
