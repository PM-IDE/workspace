#ifndef PROCFILER_SHADOWSTACKSERIALIZER_H
#define PROCFILER_SHADOWSTACKSERIALIZER_H

#include <string>
#include "../../../util/util.h"
#include "corprof.h"
#include "../ShadowStack.h"

class ShadowStackSerializer {
public:
    virtual ~ShadowStackSerializer() = default;
    virtual void Init() = 0;
    virtual void Serialize(ShadowStack* shadowStack) = 0;
};

class ShadowStackSerializerStub : public ShadowStackSerializer {
public:
    ~ShadowStackSerializerStub() override = default;

    void Init() override {};
    void Serialize(ShadowStack* shadowStack) override {};
};

#endif