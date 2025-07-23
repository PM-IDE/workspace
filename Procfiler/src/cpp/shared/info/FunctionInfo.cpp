#include "FunctionInfo.h"
#include "corprof.h"
#include <vector>


FunctionInfo FunctionInfo::GetFunctionInfo(ICorProfilerInfo15* info, FunctionID funcId) {
    mdToken functionToken;
    ClassID classId;
    ModuleID moduleId;

    info->GetFunctionInfo(funcId, &classId, &moduleId, &functionToken);

    IUnknown* unknown;
    info->GetModuleMetaData(moduleId, ofRead | ofWrite, IID_IMetaDataImport, &unknown);

    IMetaDataImport2* metadataImport = nullptr;
    void** ptr = reinterpret_cast<void**>(&metadataImport);
    unknown->QueryInterface(IID_IMetaDataImport, ptr);

    return GetFunctionInfo(metadataImport, functionToken);
}

FunctionInfo FunctionInfo::GetFunctionInfo(IMetaDataImport2* metadataImport, mdToken functionToken) {
    mdToken parentToken = mdTokenNil;
    mdToken methodSpecToken = mdTokenNil;
    mdToken methodDefToken = mdTokenNil;

    std::vector<WCHAR> functionName(MAX_CLASS_NAME, (WCHAR) 0);
    DWORD functionNameLength = 0;

    PCCOR_SIGNATURE rawSignature = NULL;
    ULONG rawSignatureLength;

    std::vector<BYTE> finalSignature{};
    std::vector<BYTE> methodSpecSignature{};

    bool isGeneric = false;

    HRESULT hr = E_FAIL;
    const auto token_type = TypeFromToken(functionToken);
    switch (token_type) {
        case mdtMemberRef:
            hr = metadataImport->GetMemberRefProps(functionToken,
                                                   &parentToken,
                                                   &functionName[0],
                                                   MAX_CLASS_NAME,
                                                   &functionNameLength,
                                                   &rawSignature,
                                                   &rawSignatureLength);
            break;
        case mdtMethodDef: {
            hr = metadataImport->GetMemberProps(functionToken,
                                                &parentToken,
                                                &functionName[0],
                                                MAX_CLASS_NAME,
                                                &functionNameLength,
                                                nullptr,
                                                &rawSignature,
                                                &rawSignatureLength,
                                                nullptr, nullptr, nullptr, nullptr, nullptr);
            break;
        }
        case mdtMethodSpec: {
            isGeneric = true;
            hr = metadataImport->GetMethodSpecProps(functionToken, &parentToken, &rawSignature, &rawSignatureLength);
            if (FAILED(hr)) {
                return {};
            }

            auto genericInfo = GetFunctionInfo(metadataImport, parentToken);
            functionName = ToRaw(genericInfo.GetName());
            functionNameLength = functionName.size();
            methodSpecToken = functionToken;
            methodDefToken = genericInfo.GetId();
            finalSignature = genericInfo.GetMethodSignature().GetRawSignature();
            methodSpecSignature = ToRaw(rawSignature, rawSignatureLength);
            break;
        }
        default: {
            break;
        }
    }
    if (FAILED(hr) || functionNameLength == 0) {
        return {};
    }

    auto attributes = ExtractAttributes(metadataImport, functionToken);
    const auto typeInfo = TypeInfo::GetTypeInfo(metadataImport, parentToken);

    if (isGeneric) {
        return {methodSpecToken, ToString(functionName, functionNameLength), typeInfo, MethodSignature(finalSignature),
                GenericMethodSignature(methodSpecSignature), methodDefToken, attributes};
    }

    return {functionToken, ToString(functionName, functionNameLength), typeInfo,
            MethodSignature(ToRaw(rawSignature, rawSignatureLength)), attributes};
}

std::unordered_set<wstring> ExtractAttributes(IMetaDataImport2* metadataImport, mdToken token) {
    std::unordered_set<wstring> attributes{};
    HRESULT hr;

#define NumItems(s) (sizeof(s) / sizeof(s[0]))

    HCORENUM customAttributeEnum = NULL;
    mdTypeRef customAttributes[10];
    ULONG count, totalCount = 1;
    while (SUCCEEDED(hr = metadataImport->EnumCustomAttributes(&customAttributeEnum, token, 0,
                                                               customAttributes, NumItems(customAttributes), &count)) &&
           count > 0) {
        for (ULONG i = 0; i < count; i++, totalCount++) {
            mdToken tkObj;
            mdToken tkType;
            const BYTE* pValue;
            ULONG cbValue;
            MDUTF8CSTR pMethName = 0;
            PCCOR_SIGNATURE pSig = 0;
            ULONG cbSig;

            std::vector<WCHAR> className(MAX_CLASS_NAME, (WCHAR) 0);
            DWORD classNameLength = 0;

            hr = metadataImport->GetCustomAttributeProps(
                    customAttributes[i],
                    &tkObj,
                    &tkType,
                    (const void**) &pValue,
                    &cbValue);

            switch (TypeFromToken(tkType)) {
                case mdtMemberRef:
                    hr = metadataImport->GetNameFromToken(tkType, &pMethName);
                    hr = metadataImport->GetMemberRefProps(tkType, &tkType, 0, 0, 0, &pSig, &cbSig);
                    break;
                case mdtMethodDef:
                    hr = metadataImport->GetNameFromToken(tkType, &pMethName);
                    hr = metadataImport->GetMethodProps(tkType, &tkType, 0, 0, 0, 0, &pSig, &cbSig, 0, 0);
                    break;
            }

            switch (TypeFromToken(tkType)) {
                case mdtTypeDef:
                    hr = metadataImport->GetTypeDefProps(tkType, &className[0], MAX_CLASS_NAME, &classNameLength, 0, 0);
                    attributes.insert(ToString(className, classNameLength));
                    break;
                case mdtTypeRef:
                    hr = metadataImport->GetTypeRefProps(tkType, 0, &className[0], MAX_CLASS_NAME, &classNameLength);
                    attributes.insert(ToString(className, classNameLength));
                    break;
            }
        }
    }
    metadataImport->CloseEnum(customAttributeEnum);

    return attributes;
}

TypeInfo FunctionInfo::ResolveParameterType(TypeInfo& typeInfo) {
    if (typeInfo.IsGenericClassRef()) {
        auto parameterType = myType.GetGenerics()[typeInfo.GetGenericRefNumber()];
        parameterType.SetRefType(typeInfo.IsRefType());
        return parameterType;
    }

    if (typeInfo.IsGenericMethodRef()) {
        auto parameterType = myFunctionSpecSignature.GetGenericsTypes()[typeInfo.GetGenericRefNumber()];
        parameterType.SetRefType(typeInfo.IsRefType());
        return parameterType;
    }

    return typeInfo;
}

std::string FunctionInfo::GetFullName() {
    return ToString(myType.GetName()) + "." + ToString(myName);
}

mdToken FunctionInfo::GetId() {
    return myId;
}

wstring FunctionInfo::GetName() const {
    return myName;
}

TypeInfo FunctionInfo::GetTypeInfo() {
    return myType;
}

MethodSignature FunctionInfo::GetMethodSignature() {
    return mySignature;
}

std::unordered_set<wstring> FunctionInfo::GetAttributes() {
    return myAttributes;
}
