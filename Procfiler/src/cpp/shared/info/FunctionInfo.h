#include <unordered_set>
#include <utility>
#include "iostream"
#include "corhlpr.h"
#include "corhdr.h"
#include "corprof.h"
#include "TypeInfo.h"
#include "MethodSignature.h"
#include "GenericMethodSignature.h"

#ifndef PROCFILER_FUNCTION_INFO_H
#define PROCFILER_FUNCTION_INFO_H

struct FunctionInfo {
private:
    mdToken myId;
    wstring myName;
    TypeInfo myType{};
    MethodSignature mySignature{};
    GenericMethodSignature myFunctionSpecSignature{};
    mdToken myMethodDefId;
    std::unordered_set<wstring> myAttributes{};

    TypeInfo ResolveParameterType(TypeInfo& typeInfo);
public:
    FunctionInfo()
            : myId(0), myName(""_W), myMethodDefId(0) {}

    FunctionInfo(mdToken id, wstring name, TypeInfo type, MethodSignature signature,
                 GenericMethodSignature functionSpecSignature, mdToken methodDefId,
                 const std::unordered_set<wstring>& attributes)
        : myId(id),
          myName(std::move(name)),
          myType(std::move(type)),
          mySignature(std::move(signature)),
          myFunctionSpecSignature(std::move(functionSpecSignature)),
          myMethodDefId(methodDefId),
          myAttributes(attributes) {}

    FunctionInfo(const mdToken id, wstring name, TypeInfo type, MethodSignature signature,
                 const std::unordered_set<wstring>& attributes)
        : myId(id),
          myName(std::move(name)),
          myType(std::move(type)),
          mySignature(std::move(signature)),
          myMethodDefId(0),
          myAttributes(attributes) {}


    static FunctionInfo GetFunctionInfo(IMetaDataImport2* metadataImport, mdToken token);
    static FunctionInfo GetFunctionInfo(ICorProfilerInfo15* info, FunctionID funcId);

    std::string GetFullName();
    mdToken GetId() const;
    wstring GetName() const;
    TypeInfo GetTypeInfo();
    MethodSignature GetMethodSignature();
    std::unordered_set<wstring> GetAttributes();
};

std::unordered_set<wstring> ExtractAttributes(IMetaDataImport2* metadataImport, mdToken token);

#endif