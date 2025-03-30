# We require CMake 3.15 because we want to use `$<CXX_COMPILER_ID:compiler_ids>`
# with multiple `compiler_ids`:
# https://cmake.org/cmake/help/v3.31/manual/cmake-generator-expressions.7.html#genex:CXX_COMPILER_ID
cmake_minimum_required(VERSION 3.15)

project(KS_TEST_CPP_STL)

find_package(Boost COMPONENTS unit_test_framework REQUIRED)
find_package(ZLIB REQUIRED)
find_package(Iconv REQUIRED)

# Enforce UTF-8 source files encoding for MSVC
add_compile_options("$<$<CXX_COMPILER_ID:MSVC>:/utf-8>")

set(CMAKE_CXX_STANDARD 98)
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

		# See https://github.com/kaitai-io/kaitai_struct_cpp_stl_runtime/blob/839e7a0752950751ad129ce96d337be4045eafee/Common.cmake#L19-L23
		-Wno-long-long

		# Clang's `-Wpedantic` under `-std=c++98` enables a Clang-specific flag
		# [`-Wc99-extensions`](https://releases.llvm.org/18.1.4/tools/clang/docs/DiagnosticsReference.html#wc99-extensions).
		# This includes `warning: empty macro arguments are a C99 feature`,
		# which was reported in every single `spec/cpp_stl_98/test_*.cpp` file
		# (except those that fail to compile with `fatal error:
		# '{format_name}.h' file not found`), always followed by one of the
		# following notes:
		#
		# 1. `/usr/local/include/boost/test/unit_test_suite.hpp:211:5: note: expanded from macro 'BOOST_AUTO_TEST_CASE'` (258 times)
		# 2. `/usr/local/include/boost/test/tools/old/interface.hpp:154:45: note: expanded from macro 'BOOST_CHECK_EQUAL'` (3165 times)
		# 3. `/usr/local/include/boost/test/tools/old/interface.hpp:208:45: note: expanded from macro 'BOOST_CHECK_CLOSE'` (64 times)
		# 4. `/tests/spec/cpp_stl_98/helpers.h:14:9: note: expanded from macro 'COMPARE_ARRAY'` (75 times)
		#
		#    Followed by:
		#
		#    `/usr/local/include/boost/test/tools/old/interface.hpp:247:5: note: expanded from macro 'BOOST_CHECK_EQUAL_COLLECTIONS'`
		#
		# The above locations correspond to Boost 1.81 that we build from source
		# in the `cpp_stl/clang3.4-linux-x86_64` Docker image like this:
		# https://github.com/kaitai-io/kaitai_struct_docker_images/blob/4dfdee62a8909555264c0aefabcc87ad1a4eed0b/src/cpp_stl/clang3.4-linux-x86_64/Dockerfile#L53-L60
		#
		# Since warnings of this particular type clutter the output a lot and
		# do not come from our own code, we turn them off.
		$<$<CXX_COMPILER_ID:AppleClang,Clang>:-Wno-c99-extensions>

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
