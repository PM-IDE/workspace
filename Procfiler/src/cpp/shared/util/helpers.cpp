#include <vector>

#include "../corprofiler/info/AssemblyInfo.h"
#include "const.h"
#include "helpers.h"
#include "util.h"

HRESULT CreateAssemblyRef(IMetaDataAssemblyEmit* metadataAssemblyEmit,
                          mdAssemblyRef* libRef,
                          const std::vector<BYTE>& public_key,
                          ASSEMBLYMETADATA metadata,
                          const wstring& assemblyName) {
    HRESULT hr = metadataAssemblyEmit->DefineAssemblyRef(
            (void*) public_key.data(),
            (ULONG) public_key.size(),
            assemblyName.c_str(), &metadata, NULL, 0, 0,
            libRef);

    return hr;
}

void GetMsCorLibRef(HRESULT& hr, IMetaDataAssemblyEmit* metadataAssemblyEmit, mdModuleRef& libRef) {
    ASSEMBLYMETADATA metadata{};
    metadata.usMajorVersion = 4;
    metadata.usMinorVersion = 0;
    metadata.usBuildNumber = 0;
    metadata.usRevisionNumber = 0;

    hr = CreateAssemblyRef(metadataAssemblyEmit,
                           &libRef,
                           std::vector<BYTE>{0xB7, 0x7A, 0x5C, 0x56, 0x19, 0x34, 0xE0, 0x89},
                           metadata,
                           mscorlib);
}

void GetWrapperRef(HRESULT& hr,
                   IMetaDataAssemblyEmit* metadataAssemblyEmit,
                   mdModuleRef& libRef,
                   const wstring& assemblyName) {
    ASSEMBLYMETADATA metadata{};
    metadata.usMajorVersion = 1;
    metadata.usMinorVersion = 0;
    metadata.usBuildNumber = 0;
    metadata.usRevisionNumber = 0;

    hr = CreateAssemblyRef(metadataAssemblyEmit, &libRef, std::vector<BYTE>(), metadata, assemblyName);
}

mdToken GetTypeToken(IMetaDataEmit2* metadataEmit, mdAssemblyRef mscorlibRef, std::vector<BYTE>& type) {
    mdToken token = mdTokenNil;
    auto iter = type.begin();

    if (*iter == ELEMENT_TYPE_BYREF) {
        std::advance(iter, 1);
    }

    PCCOR_SIGNATURE temp;

    switch (*iter) {
        case ELEMENT_TYPE_BOOLEAN:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemBoolean.data(), &token);
            break;
        case ELEMENT_TYPE_CHAR:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemChar.data(), &token);
            break;
        case ELEMENT_TYPE_I1:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemByte.data(), &token);
            break;
        case ELEMENT_TYPE_U1:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemSByte.data(), &token);
            break;
        case ELEMENT_TYPE_U2:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemUInt16.data(), &token);
            break;
        case ELEMENT_TYPE_I2:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemInt16.data(), &token);
            break;
        case ELEMENT_TYPE_I4:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemInt32.data(), &token);
            break;
        case ELEMENT_TYPE_U4:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemUInt32.data(), &token);
            break;
        case ELEMENT_TYPE_I8:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemInt64.data(), &token);
            break;
        case ELEMENT_TYPE_U8:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemUInt64.data(), &token);
            break;
        case ELEMENT_TYPE_R4:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemSingle.data(), &token);
            break;
        case ELEMENT_TYPE_R8:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemDouble.data(), &token);
            break;
        case ELEMENT_TYPE_I:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemIntPtr.data(), &token);
            break;
        case ELEMENT_TYPE_U:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemUIntPtr.data(), &token);
            break;
        case ELEMENT_TYPE_STRING:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemString.data(), &token);
            break;
        case ELEMENT_TYPE_OBJECT:
            metadataEmit->DefineTypeRefByName(mscorlibRef, SystemObject.data(), &token);
            break;
        case ELEMENT_TYPE_CLASS:
            std::advance(iter, 1);
            temp = &*iter;
            token = CorSigUncompressToken(temp);
            break;
        case ELEMENT_TYPE_VALUETYPE:
            std::advance(iter, 1);
            temp = &*iter;
            token = CorSigUncompressToken(temp);
            break;
        case ELEMENT_TYPE_GENERICINST:
        case ELEMENT_TYPE_SZARRAY:
        case ELEMENT_TYPE_MVAR:
        case ELEMENT_TYPE_VAR:
            metadataEmit->GetTokenFromTypeSpec((PCCOR_SIGNATURE) &*iter,
                                               type.size() - std::distance(type.begin(), iter), &token);
            break;
        default:
            break;
    }
    return token;
}
