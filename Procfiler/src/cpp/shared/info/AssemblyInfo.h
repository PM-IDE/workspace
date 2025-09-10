#pragma once

#include <utility>

#include "cor.h"
#include "corprof.h"
#include "../util/util.h"


struct AssemblyInfo {
private:
    AssemblyID myAssemblyId;
    wstring myName;
    ModuleID myManifestModuleId;
    AppDomainID myAppDomainId;
    wstring myAppDomainName;

public:
    AssemblyInfo() : myAssemblyId(0), myName(""_W), myManifestModuleId(0), myAppDomainId(0), myAppDomainName(""_W) {}

    AssemblyInfo(const AssemblyID id, wstring name, const ModuleID manifestModuleId, const AppDomainID appDomainId, wstring appDomainName)
        : myAssemblyId(id),
          myName(std::move(name)),
          myManifestModuleId(manifestModuleId),
          myAppDomainId(appDomainId),
          myAppDomainName(std::move(appDomainName)) {}

    static AssemblyInfo GetAssemblyInfo(ICorProfilerInfo15* info, AssemblyID assemblyId);

    AssemblyID GetAssemblyId() const;
    wstring GetName();
    ModuleID GetModuleId() const;
    AppDomainID GetAppDomainId() const;
    wstring GetAppDomainName();
};
