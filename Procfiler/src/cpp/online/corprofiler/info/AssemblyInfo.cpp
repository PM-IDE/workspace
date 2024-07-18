#include "AssemblyInfo.h"

#include <vector>

#include "../../util/util.h"


AssemblyInfo AssemblyInfo::GetAssemblyInfo(ICorProfilerInfo12* info, AssemblyID assemblyId) {
    std::vector<WCHAR> assemblyName(MAX_CLASS_NAME, (WCHAR)0);
    DWORD assemblyNameLength = 0;
    AppDomainID appDomainId;
    ModuleID manifestModuleId;

    auto hr = info->GetAssemblyInfo(assemblyId, MAX_CLASS_NAME, &assemblyNameLength,
        &assemblyName[0], &appDomainId, &manifestModuleId);

    if (FAILED(hr) || assemblyNameLength == 0) {
        return {};
    }

    std::vector<WCHAR> appDomainName(MAX_CLASS_NAME, (WCHAR)0);
    DWORD appDomainNameLength = 0;

    hr = info->GetAppDomainInfo(appDomainId, MAX_CLASS_NAME, &appDomainNameLength, &appDomainName[0], nullptr);

    if (FAILED(hr) || appDomainNameLength == 0) {
        return {};
    }

    return { assemblyId, ToString(assemblyName, assemblyNameLength), manifestModuleId, appDomainId,
            ToString(appDomainName, appDomainNameLength) };
}

AssemblyID AssemblyInfo::GetAssemblyId() {
    return myAssemblyId;
}

wstring AssemblyInfo::GetName() {
    return myName;
}

ModuleID AssemblyInfo::GetModuleId() {
    return myManifestModuleId;
}

AppDomainID AssemblyInfo::GetAppDomainId() {
    return myAppDomainId;
}

wstring AssemblyInfo::GetAppDomainName() {
    return myAppDomainName;
}