#!/usr/bin/env bash

set -o errexit

readonly WORKSPACE_ROOT="$(readlink -f $(dirname "$0"))"
readonly BUILD_TYPE=${1:-Release}

source ${WORKSPACE_ROOT}/.venv/bin/activate

conan profile detect &> /dev/null || true

conan config install \
    -tf profiles/ \
    -t dir ${WORKSPACE_ROOT}/conan/profiles/

conan install ${WORKSPACE_ROOT} \
    -of ${WORKSPACE_ROOT}/build/${BUILD_TYPE} \
    -s build_type=${BUILD_TYPE} \
    --build missing \
    -pr amd64-macos -pr apple-clang17

conan cache clean '*'

cmake -S ${WORKSPACE_ROOT} \
    -B ${WORKSPACE_ROOT}/build/${BUILD_TYPE} \
    -DCMAKE_TOOLCHAIN_FILE=build/${BUILD_TYPE}/conan_toolchain.cmake \
    -DCMAKE_BUILD_TYPE=${BUILD_TYPE}

cmake --build ${WORKSPACE_ROOT}/build/${BUILD_TYPE}/ \
    --config ${BUILD_TYPE} \
    ${@:2}
