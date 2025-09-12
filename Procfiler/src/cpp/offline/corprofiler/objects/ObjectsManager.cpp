#include "ObjectsManager.h"

#include "FunctionInfo.h"
#include "clr/profilerstring.hpp"
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
                                        ObjectID* id) const {
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

    IMetaDataImport2* metadataImport = nullptr;
    auto ptr = reinterpret_cast<void**>(&metadataImport);
    hr = unknown->QueryInterface(IID_IMetaDataImport, ptr);
    if (FAILED(hr)) {
        return false;
    }

    PCCOR_SIGNATURE signature;
    ULONG signatureLength;

    hr = metadataImport->GetMethodProps(functionToken,
                                        nullptr,
                                        nullptr,
                                        0,
                                        nullptr,
                                        nullptr,
                                        &signature,
                                        &signatureLength,
                                        nullptr,
                                        nullptr);

    if (FAILED(hr)) {
        return false;
    }

    SigFormatParserImpl sigParser;
    if (!sigParser.Parse(const_cast<sig_byte*>(signature), signatureLength)) {
        return false;
    }

    mdTypeDef typeDef;
    if (FAILED(myProfilerInfo->GetClassIDInfo(classId, &moduleId, &typeDef))) {
        return false;
    }

    if (FAILED(myProfilerInfo->GetModuleMetaData(moduleId, ofRead | ofWrite, IID_IMetaDataImport, &unknown))) {
        return false;
    }

    ptr = reinterpret_cast<void**>(&metadataImport);
    if (FAILED(unknown->QueryInterface(IID_IMetaDataImport, ptr))) {
        return false;
    }

    DWORD dwTypeDefFlags;
    hr = metadataImport->GetTypeDefProps(typeDef,
                                         nullptr,
                                         0,
                                         nullptr,
                                         &dwTypeDefFlags,
                                         nullptr);

    if (FAILED(hr) || !sigParser.HasThis()) {
        return false;
    }

    if (args->ranges->length == 0) {
        std::cout << "argumentInfo->ranges->length == 0\n";
        return false;
    }

    const auto thisId = reinterpret_cast<UINT_PTR>(*reinterpret_cast<void**>(args->ranges[0].startAddress));
    COR_PRF_GC_GENERATION_RANGE generationRange;
    const auto result = myProfilerInfo->GetObjectGeneration(thisId, &generationRange);

    std::cout << FunctionInfo::GetFunctionInfo(myProfilerInfo, funcId).GetFullName();

    if (result != S_OK) {
        std::cout << "Failed to get object generation " << result << "\n";
    } else {
        *id = thisId;
        std::cout << "::" << generationRange.generation << "::" << sigParser.HasThis() << "::" << "\n";
    }

    return true;
}
