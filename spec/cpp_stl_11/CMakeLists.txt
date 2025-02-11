cmake_minimum_required(VERSION 3.11)

project(KS_TEST_CPP_STL)

find_package(Boost COMPONENTS unit_test_framework REQUIRED)
find_package(ZLIB REQUIRED)
find_package(Iconv REQUIRED)

# Enforce UTF-8 source files encoding for MSVC
add_compile_options("$<$<CXX_COMPILER_ID:MSVC>:/utf-8>")

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(RUNTIME_SRC_PATH ../../../runtime/cpp_stl)
set(PREREQ_PATH prereq)

include(${INC_PATH})

set(PREREQ_SOURCES
	${PREREQ_PATH}/my_custom_fx.cpp
	${PREREQ_PATH}/custom_fx.cpp
	${PREREQ_PATH}/custom_fx_no_args.cpp
)

set(RUNTIME_SOURCES
	${RUNTIME_SRC_PATH}/kaitai/kaitaistream.cpp
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

target_compile_definitions(ks_tests PRIVATE -DKS_ZLIB -DKS_STR_ENCODING_ICONV)

# Maximum warnings emission
#
# See https://github.com/kaitai-io/kaitai_struct_cpp_stl_runtime/blob/dbe1ca1c946794324d0d409fe53d56a397fe8874/Common.cmake#L11-L26
target_compile_options(ks_tests PRIVATE
	$<$<CXX_COMPILER_ID:MSVC>:/W4>
	$<$<NOT:$<CXX_COMPILER_ID:MSVC>>:
		-Wall -Wextra -Wpedantic
		# See <https://gcc.gnu.org/onlinedocs/gcc-13.3.0/gcc/Warning-Options.html#index-Wstrict-aliasing_003dn>:
		#
		# > Level 1: (...) it has very few false negatives. However, it has many false positives.
		#
		# If we run into false positives in the future, we can increase the level, but let's
		# start with the most aggressive option to make sure we don't miss anything.
		-fstrict-aliasing -Wstrict-aliasing=1
	>
)

add_test(ks_tests ks_tests)

enable_testing()
