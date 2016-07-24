cmake_minimum_required(VERSION 2.6)

project(KS_TEST_CPP_STL)
find_package(Boost COMPONENTS unit_test_framework REQUIRED)

set(KS_PATH ../../compiled/cpp_stl)
set(RUNTIME_SRC_PATH ../../../runtime/cpp_stl)

set(SPEC_SOURCES
	main.cpp
	test_hello_world.cpp
	test_fixed_struct.cpp
	test_instance_std.cpp
	test_repeat_eos_u4.cpp
	test_if_struct.cpp
	test_position_abs.cpp
	test_position_in_seq.cpp
	test_repeat_n_strz.cpp
	test_term_strz.cpp
	test_expr_0.cpp
	test_expr_1.cpp
	test_process_xor_const.cpp
	test_process_xor_value.cpp
	test_process_xor4_const.cpp
	test_process_xor4_value.cpp
	test_process_rotate.cpp
	test_repeat_n_struct.cpp
	test_repeat_eos_struct.cpp
)

set(KS_SOURCES
	${KS_PATH}/hello_world.cpp
	${KS_PATH}/fixed_struct.cpp
	${KS_PATH}/instance_std.cpp
	${KS_PATH}/repeat_eos_u4.cpp
	${KS_PATH}/if_struct.cpp
	${KS_PATH}/position_abs.cpp
	${KS_PATH}/position_in_seq.cpp
	${KS_PATH}/repeat_n_strz.cpp
	${KS_PATH}/term_strz.cpp
	${KS_PATH}/expr_0.cpp
	${KS_PATH}/expr_1.cpp
	${KS_PATH}/process_xor_const.cpp
	${KS_PATH}/process_xor_value.cpp
	${KS_PATH}/process_xor4_const.cpp
	${KS_PATH}/process_xor4_value.cpp
	${KS_PATH}/process_rotate.cpp
	${KS_PATH}/repeat_n_struct.cpp
	${KS_PATH}/repeat_eos_struct.cpp
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
