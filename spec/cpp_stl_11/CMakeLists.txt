cmake_minimum_required(VERSION 3.1)

project(KS_TEST_CPP_STL)

set(CMAKE_MODULE_PATH "${PROJECT_SOURCE_DIR}/cmake")

find_package(Boost COMPONENTS unit_test_framework REQUIRED)
find_package(ZLIB REQUIRED)
find_package(Iconv REQUIRED)

# Enforce UTF-8 source files encoding for MSVC
add_compile_options("$<$<CXX_COMPILER_ID:MSVC>:/utf-8>")

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(RUNTIME_SRC_PATH ../../../runtime/cpp_stl)
set(PREREQ_PATH prereq)

include(${INC1_PATH})
include(${INC2_PATH})

set(PREREQ_SOURCES
	${PREREQ_PATH}/my_custom_fx.cpp
	${PREREQ_PATH}/custom_fx.cpp
)

set(RUNTIME_SOURCES
	${RUNTIME_SRC_PATH}/kaitai/kaitaistream.cpp
)

add_executable (ks_tests
	${RUNTIME_SOURCES}
	${PREREQ_SOURCES}
	${KS_SOURCES}
	${SPEC_SOURCES}
)

include_directories(
	"${PROJECT_BINARY_DIR}"
	"${KS_PATH}"
	"${RUNTIME_SRC_PATH}"
	"${PREREQ_PATH}"
	"${ZLIB_INCLUDE_DIRS}"
)

# NB: no quotes around variables here, as it messes up new "optimized
# A debug B" syntax which gets generated on Windows platforms
target_link_libraries(ks_tests
	${Boost_UNIT_TEST_FRAMEWORK_LIBRARY}
	${ZLIB_LIBRARIES}
	${Iconv_LIBRARIES}
)

target_compile_definitions(ks_tests PRIVATE -DKS_ZLIB -DKS_STR_ENCODING_ICONV)

add_test(ks_tests ks_tests)

enable_testing()
