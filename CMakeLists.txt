cmake_minimum_required(VERSION 3.19)
project(pacman)

set(CONAN_DISABLE_CHECK_COMPILER ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON) # compile_commands.json to make auto-complete working in vscode

IF(NOT CMAKE_BUILD_TYPE)
  SET(CMAKE_BUILD_TYPE Release)
ENDIF()

if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
  add_compile_options($<$<COMPILE_LANGUAGE:CXX>:-stdlib=libc++>)
endif()

add_compile_options($<$<COMPILE_LANGUAGE:CXX>:-std=c++20>)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/source)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

find_package(ftxui REQUIRED)

add_subdirectory(source/app)
add_subdirectory(source/test)
