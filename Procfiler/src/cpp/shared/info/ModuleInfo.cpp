#include "ModuleInfo.h"
#include "AssemblyInfo.h"


ModuleInfo ModuleInfo::GetModuleInfo(ICorProfilerInfo15* info, ModuleID moduleId) {
    std::vector<WCHAR> modulePath(MAX_CLASS_NAME, (WCHAR) 0);
    DWORD length = 0;
    LPCBYTE baseLoadAddress;
    AssemblyID assemblyId = 0;
    DWORD moduleFlags = 0;

    const HRESULT hr = info->GetModuleInfo2(moduleId, &baseLoadAddress, MAX_CLASS_NAME, &length, &modulePath[0],
                                            &assemblyId, &moduleFlags);

    if (FAILED(hr) || length == 0) {
        return {};
    }

    return { moduleId, ToString(modulePath, length), AssemblyInfo::GetAssemblyInfo(info, assemblyId), moduleFlags };
}

ModuleID ModuleInfo::GetId() const {
    return myId;
}

wstring ModuleInfo::GetPath() {
    return myPath;
}

AssemblyInfo ModuleInfo::GetAssemblyInfo() {
    return myAssembly;
}

DWORD ModuleInfo::GetFlags() const {
    return myFlags;
}
