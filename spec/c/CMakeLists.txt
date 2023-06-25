cmake_minimum_required(VERSION 3.1)

project(KS_TEST_C)

set(CMAKE_MODULE_PATH "${PROJECT_SOURCE_DIR}/cmake")

find_package(Boost COMPONENTS unit_test_framework REQUIRED)
find_package(ZLIB REQUIRED)
find_package(Iconv REQUIRED)

# Enforce UTF-8 source files encoding for MSVC
add_compile_options("$<$<CXX_COMPILER_ID:MSVC>:/utf-8>")

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror ")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall -Werror -Wno-unused-function -std=c89 -Wdeclaration-after-statement -Wformat=2")
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(RUNTIME_SRC_PATH ../../../runtime/c)
set(PREREQ_PATH prereq)

include(${INC_PATH})

set(PREREQ_SOURCES
	${PREREQ_PATH}/custom.c
)

set(RUNTIME_SOURCES
	${RUNTIME_SRC_PATH}/kaitaistruct.c
)

add_executable (ks_tests
	${RUNTIME_SOURCES}
	${PREREQ_SOURCES}
	${DISPOSABLE_SOURCES}
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

add_test(ks_tests ks_tests)

enable_testing()
