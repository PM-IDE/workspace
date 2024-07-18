#include "FunctionInfo.h"
#include "MethodSignature.h"

#include <utility>
#include "TypeInfo.h"
#include "parser.h"


MethodSignature::MethodSignature(std::vector<BYTE> rawSignature) : myRawSignature(std::move(rawSignature)) {
    auto begin = myRawSignature.begin() + 2 + (IsGeneric() ? 1 : 0);
    auto iter = begin;
    if (ParseRetType(iter)) {
        myReturnType = std::vector<BYTE>(begin, iter);

        myArgumentsOffset = std::distance(myRawSignature.begin(), iter);
    }
}

void MethodSignature::ParseArguments() {
    auto iter = myRawSignature.begin();
    std::advance(iter, myArgumentsOffset);

    for (size_t i = 0; i < NumberOfArguments(); i++) {
        auto begin = iter;
        if (!ParseParam(iter)) {
            break;
        }

        myArguments.emplace_back(std::vector<BYTE>(begin, iter));
    }
}

std::vector<BYTE> MethodSignature::GetRawSignature() {
    return myRawSignature;
}

TypeInfo MethodSignature::GetReturnTypeInfo() {
    return myReturnType;
}

std::vector<TypeInfo> MethodSignature::GetArguments() {
    return myArguments;
}

COR_SIGNATURE MethodSignature::CallingConvention() const { return myRawSignature.empty() ? 0 : myRawSignature[0]; }

bool MethodSignature::IsInstanceMethod() const {
    return (CallingConvention() & IMAGE_CEE_CS_CALLCONV_HASTHIS) != 0;
}

bool MethodSignature::IsGeneric() const {
    return myRawSignature.size() > 2 && (CallingConvention() & IMAGE_CEE_CS_CALLCONV_GENERIC) != 0;
}

ULONG MethodSignature::NumberOfArguments() const {
    if (IsGeneric()) {
        return myRawSignature[2];
    }

    if (myRawSignature.size() > 1) {
        return myRawSignature[1];
    }

    return 0;
}