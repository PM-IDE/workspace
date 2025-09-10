#pragma once

#include <utility>

#include "AssemblyInfo.h"
#include "../util/util.h"


struct ModuleInfo {
private:
    ModuleID myId;
    wstring myPath;
    AssemblyInfo myAssembly;
    DWORD myFlags;

public:
    ModuleInfo() : myId(0), myPath(""_W), myAssembly({}), myFlags(0) {}

    ModuleInfo(const ModuleID id, wstring path, AssemblyInfo assembly, const DWORD flags)
        : myId(id), myPath(std::move(path)), myAssembly(std::move(assembly)), myFlags(flags) {}


    static ModuleInfo GetModuleInfo(ICorProfilerInfo15* info, ModuleID moduleId);

    ModuleID GetId() const;
    wstring GetPath();
    AssemblyInfo GetAssemblyInfo();
    DWORD GetFlags() const;
};
