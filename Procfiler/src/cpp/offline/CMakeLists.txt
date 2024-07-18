cmake_minimum_required(VERSION 3.14.5)

add_compile_options(-fpermissive)
project(Procfiler)

set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${PROJECT_BINARY_DIR}/Release)

set(SOURCES
    dllmain.cpp
    procfiler.def
    ClassFactory.cpp
    corprofiler/ProcfilerCorProfilerCallback.cpp
    guids.cpp
    logging/ProcfilerLogger.cpp
    corprofiler/info/FunctionInfo.cpp
    corprofiler/info/TypeInfo.cpp
    corprofiler/info/MethodSignature.cpp
    corprofiler/info/MethodSignature.h
    corprofiler/info/GenericMethodSignature.cpp
    corprofiler/info/GenericMethodSignature.h
    util/const.h
    util/util.cpp
    util/helpers.cpp
    corprofiler/info/AssemblyInfo.cpp
    corprofiler/info/ModuleInfo.cpp
    corprofiler/info/parser.cpp
    miniutf/miniutf.cpp
    miniutf/miniutf.hpp
    corprofiler/shadowstack/ShadowStack.cpp
    util/env_constants.h
    corprofiler/shadowstack/serializers/EventPipeShadowStackSerializer.cpp
    corprofiler/shadowstack/serializers/BinaryShadowStackSerializer.cpp
    corprofiler/shadowstack/serializers/DebugShadowStackSerializer.cpp
    util/env_constants.cpp
    corprofiler/shadowstack/serializers/EventPipeShadowStackSerializer.h
    corprofiler/shadowstack/serializers/BinaryShadowStackSerializer.h
    corprofiler/shadowstack/serializers/DebugShadowStackSerializer.h
    corprofiler/shadowstack/utils.cpp
    corprofiler/shadowstack/EventsWithThreadId.cpp
)

include_directories(../3rdparty/coreclr/pal/prebuilt/inc)
include_directories(../3rdparty/coreclr/inc)

if(NOT WIN32)
    include_directories(../3rdparty/coreclr/pal/inc/rt
                        ../3rdparty/coreclr/pal/inc
                        ../3rdparty/coreclr/pal/src/include/)

    add_compile_options(-DPAL_STDCPP_COMPAT)

    add_compile_options(-std=c++11 -Wno-invalid-noreturn -Wno-macro-redefined -Wno-pragma-pack
                        -DHOST_64BIT -DPAL_STDCPP_COMPAT -DPLATFORM_UNIX -DUNICODE
                        -fPIC -fms-extensions)

    IF (APPLE)
        add_compile_options(-stdlib=libc++ -DHOST_ARM64 -DHAVE_CLOCK_GETTIME_NSEC_NP)
    ELSE ()
        add_compile_options(-DHOST_AMD64 -DHAVE_CLOCK_MONOTONIC)
    ENDIF ()

    if(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
        add_compile_options(-Wno-null-arithmetic)
    else(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
        add_compile_options(-Wno-conversion-null -Wno-pointer-arith)
    endif(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
endif(NOT WIN32)

add_library(Procfiler SHARED ${SOURCES})
set_target_properties(Procfiler PROPERTIES PREFIX "")
set_target_properties(Procfiler PROPERTIES OUTPUT_NAME "CppProcfiler")
set_target_properties(Procfiler PROPERTIES SUFFIX ".dll")

if(WIN32)
    set(DEPENDENCIES Advapi32.lib oleaut32.lib Ole32.lib)
else(WIN32)
    set(DEPENDENCIES pthread dl)
endif(WIN32)

target_link_libraries(Procfiler PUBLIC ${DEPENDENCIES})

install(TARGETS Procfiler DESTINATION bin)

if(WIN32)
    install(FILES $<TARGET_PDB_FILE:${PROJECT_NAME}> DESTINATION bin OPTIONAL)
endif(WIN32)