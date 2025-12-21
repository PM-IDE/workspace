#pragma once

#include <utility>

#include "cor.h"
#include "../util/util.h"

struct TypeInfo {
private:
    mdToken myToken;
    wstring myName;
    std::vector<BYTE> myRaw{};

    std::vector<TypeInfo> myGenerics{};

    bool myIsRefType = false;
    bool myIsClass = false;
    BYTE myTypeDef = 0;
    bool myIsBoxed = false;
    bool myIsVoid = false;

    bool myIsGenericClassRef = false;
    bool myIsGenericMethodRef = false;
    ULONG myGenericRefNumber = 0;

public:
    TypeInfo() : myToken(0), myName(""_W) {}

    TypeInfo(const mdToken id, wstring name, const std::vector<BYTE>& raw) : myToken(id), myName(std::move(name)), myRaw(raw) {}

    TypeInfo(const std::vector<BYTE>& raw);

    void TryParseGeneric();

    static TypeInfo GetTypeInfo(IMetaDataImport2* metadataImport, mdToken token);

    mdToken GetToken() const;
    wstring GetName();
    std::vector<BYTE> GetRawInfo();
    std::vector<TypeInfo> GetGenerics();

    bool IsRefType() const;
    void SetRefType(bool isRefType);
    bool IsClass() const;

    bool IsBoxed() const;
    bool IsVoid() const;

    bool IsGenericClassRef() const;
    bool IsGenericMethodRef() const;

    BYTE GetTypeDef() const;
    ULONG GetGenericRefNumber() const;
};