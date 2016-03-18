cmake_minimum_required(VERSION 2.6)

project(KS_TEST_CPP_STL)
find_package(Boost COMPONENTS unit_test_framework REQUIRED)

set(KS_PATH ../../compiled/cpp_stl)
set(RUNTIME_SRC_PATH ../../../runtime/cpp_stl)

set(SPEC_SOURCES
	main.cpp
	test_hello_world.cpp
	test_fixed_struct.cpp
)

set(KS_SOURCES
	${KS_PATH}/hello_world.cpp
	${KS_PATH}/fixed_struct.cpp
)

set(RUNTIME_SOURCES
	${RUNTIME_SRC_PATH}/kaitai/kaitaistream.cpp
)

add_executable (ks_tests
	${SPEC_SOURCES}
	${KS_SOURCES}
	${RUNTIME_SOURCES}
)

include_directories("${PROJECT_BINARY_DIR}" "${KS_PATH}" "${RUNTIME_SRC_PATH}")

target_link_libraries(ks_tests
	${Boost_UNIT_TEST_FRAMEWORK_LIBRARY}
)

add_test(ks_tests ks_tests)

enable_testing()
