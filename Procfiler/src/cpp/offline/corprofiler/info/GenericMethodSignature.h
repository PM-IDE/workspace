#pragma once

#include "../../util/util.h"
#include "TypeInfo.h"


struct GenericMethodSignature {
private:
    std::vector<BYTE> myRawSignature{};
    std::vector<TypeInfo> myGenerics{};

public:
    explicit GenericMethodSignature(std::vector<BYTE> rawSignature);

    GenericMethodSignature() = default;

    std::vector<BYTE> GetRawSignature();
    std::vector<TypeInfo> GetGenericsTypes();
};