# CMakeList.txt: StartingPkg 的 CMake 项目，在此处包括源代码并定义
# 项目特定的逻辑。
#
cmake_minimum_required (VERSION 3.8)

project ("StartingPkg" LANGUAGES CXX VERSION 1)

# 将源代码添加到此项目的可执行文件。

if(UNIX)
add_compile_options("-nostd")
add_compile_options("-nostdlib")
add_compile_options("-nostartfiles")
endif()

if(WIN32)
add_link_options("/NXCOMPAT:NO")
add_link_options("/NODEFAULTLIB")
add_compile_options("/GS-")
add_compile_options("/EHs-c-")
endif()

add_library(Starting SHARED StartingPkg.cpp)

target_include_directories(Starting PUBLIC "${UEFI_INCLUDE_DIR}")
target_include_directories(Starting PUBLIC "${UEFI_INCLUDE_DIR}/X64")
target_include_directories(Starting PUBLIC "${UEFI_INCLUDE_DIR}/Library")
target_include_directories(Starting PUBLIC "${UEFI_INCLUDE_DIR}/Uefi")


target_link_libraries(Starting "${UEFI_LINK_LIB}")


# TODO: 如有需要，请添加测试并安装目标。
