cmake_minimum_required(VERSION 3.21.0)

project(depfile_test
    LANGUAGES C
    DESCRIPTION "A CMake Project to test the --depfile output from cbindgen"
    )

include(../cbindgen_test.cmake)

add_cbindgen_command(gen_bindings
    "${CMAKE_CURRENT_BINARY_DIR}/single_crate.h"
    )
