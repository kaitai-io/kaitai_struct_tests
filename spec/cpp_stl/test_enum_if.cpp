#include <boost/test/unit_test.hpp>

#include <enum_if.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_enum_if) {
    std::ifstream ifs("src/if_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    enum_if_t* r = new enum_if_t(&ks);

    BOOST_CHECK_EQUAL(r->op1()->opcode(), enum_if_t::OPCODES_A_STRING);
    BOOST_CHECK_EQUAL(r->op1()->arg_str()->str(), "foo");

    BOOST_CHECK_EQUAL(r->op2()->opcode(), enum_if_t::OPCODES_A_TUPLE);
    BOOST_CHECK_EQUAL(r->op2()->arg_tuple()->num1(), 0x42);
    BOOST_CHECK_EQUAL(r->op2()->arg_tuple()->num2(), 0x43);

    BOOST_CHECK_EQUAL(r->op3()->opcode(), enum_if_t::OPCODES_A_STRING);
    BOOST_CHECK_EQUAL(r->op3()->arg_str()->str(), "bar");

    delete r;
}
