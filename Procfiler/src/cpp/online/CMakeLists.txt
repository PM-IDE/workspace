cmake_minimum_required(VERSION 3.14.5)

add_compile_options(-fpermissive)
project(ProcfilerOnline)

set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${PROJECT_BINARY_DIR}/Release)

set(SOURCES
    dllmain.cpp
    procfiler.def
    ClassFactory.cpp
    guids.cpp
    corprofiler/ProcfilerCorProfilerCallback.cpp
    corprofiler/eventpipe/EventPipeWriter.cpp
    ../shared/info/FunctionInfo.cpp
    ../shared/info/TypeInfo.cpp
    ../shared/info/MethodSignature.cpp
    ../shared/info/MethodSignature.h
    ../shared/info/GenericMethodSignature.cpp
    ../shared/info/GenericMethodSignature.h
    ../shared/info/clr/profilerstring.hpp
    ../shared/info/clr/profiler.hpp
    ../shared/info/clr/holder.hpp
    ../shared/info/clr/shutdownguard.hpp
    ../shared/util/const.h
    ../shared/util/util.cpp
    ../shared/util/helpers.cpp
    ../shared/info/AssemblyInfo.cpp
    ../shared/info/ModuleInfo.cpp
    ../shared/info/parser.cpp
    ../shared/miniutf/miniutf.cpp
    ../shared/miniutf/miniutf.hpp
    ../shared/util/env_constants.h
    ../shared/util/env_constants.cpp
    ../shared/util/performance_counter.h
    ../shared/util/logging/ProcfilerLogger.cpp
    corprofiler/eventpipe/FunctionEvent.h
)

include_directories(../3rdparty/coreclr/pal/prebuilt/inc)
include_directories(../3rdparty/coreclr/inc)
include_directories(../shared/miniutf)
include_directories(../shared/info)
include_directories(../shared/util)

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

add_library(ProcfilerOnline SHARED ${SOURCES})
set_target_properties(ProcfilerOnline PROPERTIES PREFIX "")
set_target_properties(ProcfilerOnline PROPERTIES OUTPUT_NAME "CppProcfilerOnline")
set_target_properties(ProcfilerOnline PROPERTIES SUFFIX ".dll")

if(WIN32)
    set(DEPENDENCIES Advapi32.lib oleaut32.lib Ole32.lib)
else(WIN32)
    set(DEPENDENCIES pthread dl)
endif(WIN32)

target_link_libraries(ProcfilerOnline PUBLIC ${DEPENDENCIES})

install(TARGETS ProcfilerOnline DESTINATION bin)

if(WIN32)
    install(FILES $<TARGET_PDB_FILE:${PROJECT_NAME}> DESTINATION bin OPTIONAL)
endif(WIN32)