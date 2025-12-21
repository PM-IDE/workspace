#ifndef PROCFILER_SIGPARSERIMPL_H
#define PROCFILER_SIGPARSERIMPL_H

#include "sigparser.h"

class SigFormatParserImpl final : public SigParser {
    bool myHasThis = false;

public:
    bool HasThis() const {
        return myHasThis;
    }

protected:
    void NotifyHasThis() override {
        myHasThis = true;
    }

    void NotifyBeginMethod(sig_elem_type elem_type) override {
    }

    void NotifyEndMethod() override {
    }

    void NotifyParamCount(sig_count) override {
    }

    void NotifyBeginRetType() override {
    }

    void NotifyEndRetType() override {
    }

    void NotifyBeginParam() override {
    }

    void NotifyEndParam() override {
    }

    void NotifySentinel() override {
    }

    void NotifyGenericParamCount(sig_count) override {
    }

    void NotifyBeginField(sig_elem_type elem_type) override {
    }

    void NotifyEndField() override {
    }

    void NotifyBeginLocals(sig_elem_type elem_type) override {
    }

    void NotifyEndLocals() override {
    }

    void NotifyLocalsCount(sig_count) override {
    }

    void NotifyBeginLocal() override {
    }

    void NotifyEndLocal() override {
    }

    void NotifyConstraint(sig_elem_type elem_type) override {
    }

    void NotifyBeginProperty(sig_elem_type elem_type) override {
    }

    void NotifyEndProperty() override {
    }

    void NotifyBeginArrayShape() override {
    }

    void NotifyEndArrayShape() override {
    }

    void NotifyRank(sig_count) override {
    }

    void NotifyNumSizes(sig_count) override {
    }

    void NotifySize(sig_count) override {
    }

    void NotifyNumLoBounds(sig_count) override {
    }

    void NotifyLoBound(sig_count) override {
    }

    void NotifyBeginType() override {
    }

    void NotifyEndType() override {
    }

    void NotifyTypedByref() override {
    }

    void NotifyByref() override {
    }

    void NotifyVoid() override {
    }

    void NotifyCustomMod(sig_elem_type cmod, sig_index_type indexType, sig_index index) override {
    }

    void NotifyTypeSimple(sig_elem_type elem_type) override {
    }

    void NotifyTypeDefOrRef(sig_index_type indexType, int index) override {
    }

    void NotifyTypeGenericInst(sig_elem_type elem_type, sig_index_type indexType, sig_index index,
                               sig_mem_number number) override {
    }

    void NotifyTypeGenericTypeVariable(sig_mem_number number) override {
    }

    void NotifyTypeGenericMemberVariable(sig_mem_number number) override {
    }

    void NotifyTypeValueType() override {
    }

    void NotifyTypeClass() override {
    }

    void NotifyTypePointer() override {
    }

    void NotifyTypeFunctionPointer() override {
    }

    void NotifyTypeArray() override {
    }

    void NotifyTypeSzArray() override {
    }
};

#endif //PROCFILER_SIGPARSERIMPL_H
