#pragma once

#include "cor.h"
#include "corprof.h"
#include "holder.hpp"
#include "profilerstring.hpp"

#define SHORT_LENGTH    32
#define STR_LENGTH     256
#define LONG_LENGTH   1024

// copy-paste from https://github.com/dotnet/runtime/tree/main/src/tests/profiler/native
inline String GetClassIdName(const ClassID classId, ICorProfilerInfo15 *info) {
    if (classId == NULL) {
        return WCHAR("");
    }

    ModuleID modId;
    mdTypeDef classToken;
    ClassID parentClassID;
    ULONG32 nTypeArgs;
    ClassID typeArgs[SHORT_LENGTH];
    HRESULT hr = S_OK;

    hr = info->GetClassIDInfo2(classId, &modId, &classToken, &parentClassID, SHORT_LENGTH, &nTypeArgs, typeArgs);

    if (CORPROF_E_CLASSID_IS_ARRAY == hr) {
        // We have a ClassID of an array.
        return WCHAR("");
    }

    if (CORPROF_E_CLASSID_IS_COMPOSITE == hr) {
        // We have a composite class
        return WCHAR("");
    }

    if (CORPROF_E_DATAINCOMPLETE == hr) {
        // type-loading is not yet complete. Cannot do anything about it.
        return WCHAR("");
    }

    if (FAILED(hr)) {
        return WCHAR("");
    }

    COMPtrHolder<IMetaDataImport> metadataImport;
    hr = info->GetModuleMetaData(modId,
                                 ofRead | ofWrite,
                                 IID_IMetaDataImport,
                                 reinterpret_cast<IUnknown**>(&metadataImport));

    if (FAILED(hr)) {
        // ClassIDLookupFailed
        return WCHAR("");
    }

    WCHAR wName[LONG_LENGTH];
    DWORD dwTypeDefFlags = 0;
    String name = WCHAR("");

    auto currentClassToken = classToken;
    while (true) {
        mdTypeDef nestedDef;
        hr = metadataImport->GetNestedClassProps(currentClassToken, &nestedDef);
        if (FAILED(hr)) {
            break;
        }

        if (metadataImport->IsValidToken(nestedDef)) {
            hr = metadataImport->GetTypeDefProps(nestedDef, wName, LONG_LENGTH, NULL, &dwTypeDefFlags, NULL);

            if (FAILED(hr)) {
                break;
            }

            String newName = WCHAR("");
            newName += wName;

            if (name.Length() > 0) {
                newName += WCHAR("+");
                newName += name;
            }

            name = newName;
            currentClassToken = nestedDef;
        } else {
            break;
        }
    }

    hr = metadataImport->GetTypeDefProps(classToken, wName, LONG_LENGTH, NULL, &dwTypeDefFlags, NULL);

    if (FAILED(hr)) {
        return WCHAR("");
    }

    if (name.Length() > 0) {
        name += WCHAR("+");
    }

    name += wName;

    if (nTypeArgs > 0)
        name += WCHAR("<");

    for (ULONG32 i = 0; i < nTypeArgs; i++) {
        String typeArgClassName;
        typeArgClassName.Clear();
        name += GetClassIdName(typeArgs[i], info);

        if (i + 1 != nTypeArgs)
            name += WCHAR(", ");
    }

    if (nTypeArgs > 0)
        name += WCHAR(">");

    return name;
}

inline String GetFunctionIdName(const FunctionID funcId, ICorProfilerInfo15 *info) {
    // If the FunctionID is 0, we could be dealing with a native function.
    if (funcId == 0) {
        return WCHAR("");
    }

    String name;

    ClassID classId = NULL;
    ModuleID moduleId = NULL;
    mdToken token = NULL;
    ULONG32 nTypeArgs = NULL;
    ClassID typeArgs[SHORT_LENGTH];
    constexpr COR_PRF_FRAME_INFO frameInfo = NULL;

    HRESULT hr = S_OK;
    hr = info->GetFunctionInfo2(funcId, frameInfo, &classId, &moduleId, &token, SHORT_LENGTH, &nTypeArgs, typeArgs);

    if (FAILED(hr)) {
        // FuncNameLookupFailed
        return WCHAR("");
    }

    COMPtrHolder<IMetaDataImport> pIMDImport;
    hr = info->GetModuleMetaData(moduleId,
                                 ofRead,
                                 IID_IMetaDataImport,
                                 reinterpret_cast<IUnknown**>(&pIMDImport));

    if (FAILED(hr)) {
        // FuncNameLookupFailed
        return WCHAR("");
    }

    WCHAR funcName[STR_LENGTH];
    hr = pIMDImport->GetMethodProps(token, NULL, funcName, STR_LENGTH, 0, 0, NULL, NULL, NULL, NULL);

    if (FAILED(hr)) {
        // FuncNameLookupFailed
        return WCHAR("");
    }

    name += funcName;

    // Fill in the type parameters of the generic method
    if (nTypeArgs > 0) {
        name += WCHAR("<");
    }

    for (ULONG32 i = 0; i < nTypeArgs; i++) {
        name += GetClassIdName(typeArgs[i], info);

        if (i + 1 != nTypeArgs) {
            name += WCHAR(", ");
        }
    }

    if (nTypeArgs > 0) {
        name += WCHAR(">");
    }

    return name;
}

inline String GetModuleIDName(const ModuleID modId, ICorProfilerInfo15 *info) {
    WCHAR moduleName[STR_LENGTH];
    ULONG nameLength = 0;
    AssemblyID assemID;

    if (modId == NULL) {
        return WCHAR("");
    }

    const HRESULT hr = info->GetModuleInfo(modId, NULL, STR_LENGTH, &nameLength, moduleName, &assemID);
    if (FAILED(hr)) {
        return WCHAR("");
    }

    return moduleName;
}

inline wstring GetFullFunctionName(const FunctionID functionId, ICorProfilerInfo15* info) {
    mdToken functionToken;
    ClassID classId;
    ModuleID moduleId;

    const HRESULT hr = info->GetFunctionInfo(functionId, &classId, &moduleId, &functionToken);
    if (FAILED(hr)) {
        return WCHAR("");
    }

    auto functionName = GetClassIdName(classId, info);
    if (functionName.Length() > 0) {
        functionName += WCHAR(".");
    }

    functionName += GetFunctionIdName(functionId, info);

    return functionName.ToWString();
}