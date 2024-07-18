#pragma once

#include "cor.h"
#include "corprof.h"
#include "../corprofiler/info/FunctionInfo.h"
#include "../corprofiler/info/ModuleInfo.h"


void GetMsCorLibRef(HRESULT& hr, IMetaDataAssemblyEmit* metadataAssemblyEmit, mdModuleRef& libRef);
void GetWrapperRef(HRESULT& hr,
                   IMetaDataAssemblyEmit* metadataAssemblyEmit,
                   mdModuleRef& libRef,
                   const wstring& assemblyName);

mdToken GetTypeToken(IMetaDataEmit2* metadataEmit, mdAssemblyRef mscorlibRef, std::vector<BYTE>& type);
