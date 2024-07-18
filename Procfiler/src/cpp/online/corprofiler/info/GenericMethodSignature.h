#pragma once

#include <vector>

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