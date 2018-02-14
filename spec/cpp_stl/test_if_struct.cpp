#include <boost/test/unit_test.hpp>

#include <if_struct.h>

#include <iostream>
#include <fstream>

BOOST_AUTO_TEST_CASE(test_if_struct) {
    std::ifstream ifs("src/if_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    if_struct_t* r = new if_struct_t(&ks);

    BOOST_CHECK_EQUAL(r->op1()->opcode(), 0x53);
    BOOST_CHECK_EQUAL(r->op1()->arg_str()->str(), "foo");

    BOOST_CHECK_EQUAL(r->op2()->opcode(), 0x54);
    BOOST_CHECK_EQUAL(r->op2()->arg_tuple()->num1(), 0x42);
    BOOST_CHECK_EQUAL(r->op2()->arg_tuple()->num2(), 0x43);

    BOOST_CHECK_EQUAL(r->op3()->opcode(), 0x53);
    BOOST_CHECK_EQUAL(r->op3()->arg_str()->str(), "bar");

    delete r;
}
